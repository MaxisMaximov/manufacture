use std::io::Write;

use crossterm::style::Stylize;

use super::*;

pub struct sys_Renderer{
    frameBuffer: DoubleDArray<StyleSet>
}
impl<'a> gmSystem<'a> for sys_Renderer{
    type sysData = sysData_Renderer<'a>;

    fn new() -> Self {
        Self{
            frameBuffer: DoubleDArray::new(RENDER_BUFFER_X, RENDER_BUFFER_Y)
        }
    }

    fn SYS_ID() -> &'static str {
        "sys_Renderer"
    }

    fn execute(&mut self, IN_data: Self::sysData) {

        // First check if an active camera exists
        let mut w_camera: Option<&comp_ViewportCamera> = None;

        for CAM in IN_data.comp_ViewportCamera.inner.iter(){
            if !CAM.val.active{continue}
            w_camera = Some(&CAM.val);
        }

        if let Some(VIEWPORT) = w_camera{

            // Set up the buffers
            let mut w_WorldBuffer: DoubleDArray<StyleSet> = DoubleDArray::new(RENDER_VIEWPORT_X, RENDER_VIEWPORT_Y);
            let mut w_WorldZBuffer: DoubleDArray<u16> = DoubleDArray::new(RENDER_VIEWPORT_X, RENDER_VIEWPORT_Y);

            // Get the tracked entity position
            // Since Chunks are also entities you can attach cameras to them too, fun fact
            let w_trackPos = IN_data.comp_Pos.get(&VIEWPORT.trackedEntity);

            // Set boundaries
            let w_minCoords: Vector2 = (
                // Position + World Offset + Lower Boundary Offset + Margin
                w_trackPos.x + VIEWPORT.offset.0 + RENDER_VIEWPORT_X_MIN - RENDER_MARGIN,
                w_trackPos.y + VIEWPORT.offset.1 + RENDER_VIEWPORT_Y_MIN - RENDER_MARGIN
            );
            let w_maxCoords: Vector2 = (
                w_trackPos.x + VIEWPORT.offset.0 + RENDER_VIEWPORT_X_MAX + RENDER_MARGIN,
                w_trackPos.y + VIEWPORT.offset.1 + RENDER_VIEWPORT_Y_MAX + RENDER_MARGIN
            );

            for OBJ in IN_data.comp_Sprite.inner.iter(){
                
                let w_objPos = IN_data.comp_Pos.get(&OBJ.id);

                // If it's outside the boundaries on ANY axis, ignore it
                // It's a mess, I know
                if w_objPos.x < w_minCoords.0 || w_objPos.x > w_maxCoords.0
                    || w_objPos.y < w_minCoords.1 || w_objPos.y > w_maxCoords.1{
                        continue
                    }

                // Find offset relative to the camera
                let w_objOffset: Vector2 = (w_objPos.x - w_trackPos.x , w_objPos.y - w_trackPos.y);

                // Set an iterator to avoid the Zip() spaghett
                let mut SPRITE_PIXELS = OBJ.val.sprite.iter();

                // Settings iterators cuz there's no such thing as Range Offset, or Range(Start, Length), sad
                let w_iterStart: Vector2 = (w_objOffset.0 - (OBJ.val.sizeX / 2) as isize, w_objOffset.1 + (OBJ.val.sizeY / 2) as isize);
                
                let w_iterEnd: Vector2 = (w_iterStart.0 + OBJ.val.sizeX as isize, w_iterStart.1 - OBJ.val.sizeY as isize);

                // And finally iterate
                // Traverses the sprite Top>>Bottom, because that's how images are stored
                // Easier to do an iterator than some dark magic peckneckiry to rearrange images
                for YPOS in (w_iterEnd.1..w_iterStart.1).rev(){
                    for XPOS in w_iterStart.0..w_iterEnd.0{
                        if !w_WorldZBuffer[(XPOS, YPOS)] < OBJ.val.zDepth{continue}

                        w_WorldBuffer[(XPOS, YPOS)] = *SPRITE_PIXELS.next().unwrap();

                        w_WorldZBuffer[(XPOS, YPOS)] = OBJ.val.zDepth;
                    }
                }
            }

            // Now paste all that into the main buffer
            // DoubleDArray stores everything Bottom>>Top so I can iterate over it directly
            let mut BUFFER_ROWITER = w_WorldBuffer.inner.chunks(RENDER_VIEWPORT_X);


            for YPOS in RENDER_VIEWPORT_Y_MIN..=RENDER_VIEWPORT_Y_MAX{
                let mut ROW_ITER = BUFFER_ROWITER.next().unwrap().iter();

                for XPOS in RENDER_VIEWPORT_X_MIN..=RENDER_VIEWPORT_X_MAX{
                    self.frameBuffer[(XPOS, YPOS)] = *ROW_ITER.next().unwrap()
                }
            }
        }

        // Render the UI boxes
        for UIBOX in IN_data.comp_UIBox.inner.iter(){
            self.renderNode(&UIBOX.val.elements, &UI_data { position: (0, 0) });
        }

        // Lock the Output
        let mut STDLOCK = std::io::BufWriter::new(std::io::stdout().lock());

        // Buffer the frame into string output
        for ROW in self.frameBuffer.inner.chunks(RENDER_BUFFER_X).rev(){
            for CELL in ROW.iter(){
                let _ = STDLOCK.write(CELL.ch.with(CELL.fg).on(CELL.bg).to_string().as_bytes());
            }
        }

        // And finally print the frame
        let _ = STDLOCK.flush();

        // And drop the lock
        drop(STDLOCK)
    }
}
impl sys_Renderer{
    pub fn renderNode(&mut self, IN_node: &Node<UI_element>, IN_uiData: &UI_data){
        let w_startPos = match IN_node.position{
            UI_pos::Abs(POS) => (POS.0, POS.1),
            UI_pos::Rel(POS) => (POS.0 + IN_uiData.position.0, POS.1 + IN_uiData.position.1)
        
        };

        let mut w_pos = w_startPos;

        for CHAR in IN_node.content.chars(){
            // Hacky workaround
            if CHAR == '\n'{
                w_pos.1 += 1;
                w_pos.0 = w_startPos.0;
                continue
            }
            
            let mut idkfa_cell = self.frameBuffer[w_pos];
            idkfa_cell.ch = CHAR;
            idkfa_cell.fg = IN_node.fg;
            idkfa_cell.bg = IN_node.bg;
            w_pos.0 += 1;
        }

        let w_nextUIData = UI_data{
            position: w_startPos,
        };

        for NODE in IN_node.nodes.iter(){
            self.renderNode(NODE, &w_nextUIData);
        }
    }
}

pub struct sysData_Renderer<'a>{
    pub comp_Pos: ReadStorage<'a, comp_Pos>,
    pub comp_Sprite: ReadStorage<'a, comp_Sprite>,
    pub comp_ViewportCamera: ReadStorage<'a, comp_ViewportCamera>,
    pub comp_UIBox: ReadStorage<'a, comp_UIBox>,
    pub res_UIData: Fetch<'a, res_UIData>
}
impl<'a> gmSystemData<'a> for sysData_Renderer<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_Pos: IN_world.fetch(),
            comp_Sprite: IN_world.fetch(),
            comp_ViewportCamera: IN_world.fetch(),
            comp_UIBox: IN_world.fetch(),
            res_UIData: IN_world.fetchRes()
        }
    }
}