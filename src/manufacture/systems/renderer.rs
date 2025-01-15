use std::io::{BufWriter, Write};

use crossterm::style::Stylize;

use super::*;

pub struct sys_Renderer{
    frameBuffer: CartesianGrid<StyleSet>
}
impl<'a> gmSystem<'a> for sys_Renderer{
    type sysData = sysData_Renderer<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{
            frameBuffer: CartesianGrid::new(RENDER_BUFFER_X, RENDER_BUFFER_Y)
        }
    }

    fn SYS_ID() -> &'static str {
        "sys_Renderer"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {

        // First check if an active camera exists
        let mut w_camera: Option<&comp_ViewportCamera> = None;

        for CAM in IN_data.comp_ViewportCamera.inner.iter(){
            if !CAM.val.active{continue}
            w_camera = Some(&CAM.val);
        }

        // Render world
        if let Some(VIEWPORT) = w_camera{

            // Set up the buffers
            let mut w_WorldBuffer: CartesianGrid<StyleSet> = CartesianGrid::new(RENDER_VIEWPORT_X, RENDER_VIEWPORT_Y);
            let mut w_WorldZBuffer: CartesianGrid<u16> = CartesianGrid::new(RENDER_VIEWPORT_X, RENDER_VIEWPORT_Y);

            // Get the tracked entity position
            // Since Chunks are also entities you can attach cameras to them too, fun fact
            let w_trackPos = IN_data.comp_Pos.get(&VIEWPORT.trackedEntity).expect("ERROR: Viewport camera is tracking a nonexistent entity");

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
                
                // If it doesn't have a position then set it to 0, 0
                // Sprites don't *need* position really
                let w_objPos = IN_data.comp_Pos.get(&OBJ.id).unwrap_or(&comp_Pos { x: 0, y: 0 });

                // If it's outside the boundaries on ANY axis, ignore it
                // It's a mess, I know
                if w_objPos.x < w_minCoords.0 || w_objPos.x > w_maxCoords.0
                    || w_objPos.y < w_minCoords.1 || w_objPos.y > w_maxCoords.1{
                        continue
                    }

                // Find offset relative to the camera
                let w_objOffset: Vector2 = (w_objPos.x - w_trackPos.x , w_objPos.y - w_trackPos.y);

                // Set an iterator to avoid the Zip() spaghett
                let mut SPRITE_PIXELS = OBJ.val.sprite.chunks(OBJ.val.sizeX).rev();

                // Setting iterators cuz there's no such thing as Range Offset, or Range(Start, Length), sad
                let w_iterStart: Vector2 = (w_objOffset.0 - (OBJ.val.sizeX / 2) as isize, w_objOffset.1 + (OBJ.val.sizeY / 2) as isize); // TL
                
                let w_iterEnd: Vector2 = (w_iterStart.0 + OBJ.val.sizeX as isize, w_iterStart.1 - OBJ.val.sizeY as isize); // BR

                // And finally iterate
                for YPOS in w_iterEnd.1..w_iterStart.1{
                    let mut ROW_ITER = SPRITE_PIXELS.next().unwrap().iter();

                    for XPOS in w_iterStart.0..w_iterEnd.0{
                        if w_WorldZBuffer[(XPOS, YPOS)] > OBJ.val.zDepth{
                            ROW_ITER.next(); // Skip over a pixel if it can't be drawn
                            continue
                        }

                        w_WorldBuffer[(XPOS, YPOS)] = *ROW_ITER.next().unwrap();

                        w_WorldZBuffer[(XPOS, YPOS)] = OBJ.val.zDepth;
                    }
                }
                
            }

            // Now paste all that into the main buffer
            // DoubleDArray stores everything Bottom>>Top so I can iterate over it directly
            let mut BUFFER_ROWITER = w_WorldBuffer.inner.chunks(RENDER_VIEWPORT_X);


            for YPOS in RENDER_VIEWPORT_Y_MIN..RENDER_VIEWPORT_Y_MAX{
                let mut ROW_ITER = BUFFER_ROWITER.next().unwrap().iter();

                for XPOS in RENDER_VIEWPORT_X_MIN..RENDER_VIEWPORT_X_MAX{
                    self.frameBuffer[(XPOS, YPOS)] = *ROW_ITER.next().unwrap()
                }
            }
        
        }

        // Render the UI boxes
        for UIBOX in IN_data.comp_UIBox.inner.iter_mut(){
            self.renderUINode(
                &mut UIBOX.val.elements, 
                &UI_parentData{
                    position: (0, 0),
                    size: (RENDER_BUFFER_X, RENDER_BUFFER_Y)
                }, 
                &IN_data.res_UIData);
        }

        // THIS NEXT PART DOESN'T QUITE WORK
        // In all seriousness WHY DOES CROSSTERM'S CONVOLUTED FORMATTING WORK PERFECTLY with frame sync
        // But when *I* am trying to do it RAW it DOESN'T????

        // "Start" sync (Doesn't work for some reason)
        println!("\x1b[?2026h");
        
        // Lock the Output
        let mut STDLOCK = BufWriter::new(std::io::stdout().lock());

        // Go to the start of the screen
        STDLOCK.write(b"\x1b[H");

        // Buffer the frame into output
        for ROW in self.frameBuffer.inner.chunks(RENDER_BUFFER_X).rev(){
            for CELL in ROW.iter(){
                STDLOCK.write(CELL.ch.with(CELL.fg).on(CELL.bg).to_string().as_bytes());
            }
            STDLOCK.write(b"\r\n");
        }
        

        // "End" sync and finally print the frame
        STDLOCK.write(b"\x1b[?2026l");
        STDLOCK.flush();

        // And drop the lock
        drop(STDLOCK);

    }
}
impl sys_Renderer{
    pub fn renderUINode(&mut self, IN_node: &mut Node<UI_element>, IN_parentUiData: &UI_parentData, IN_resUIData: &res_UIData){
        let w_NodeUIData = IN_parentUiData.concatStyle(&IN_node.style);

        let idkfa_nodeTag = IN_node.tag.take();

        match &idkfa_nodeTag{
            UI_tag::none => {}
            UI_tag::text(TEXT) => {
                let mut w_charPos = w_NodeUIData.position;
                for CHAR in TEXT.chars(){
                    // Hacky workaround
                    if CHAR == '\n'{
                        w_charPos.1 -= 1;
                        w_charPos.0 = w_NodeUIData.position.0;
                        continue
                    }
                    
                    self.frameBuffer[w_charPos] = StyleSet{
                        ch: CHAR,
                        fg: IN_node.style.fg,
                        bg: IN_node.style.bg,
                    };
                    w_charPos.0 += 1;
                }
            },
            UI_tag::special(SPECIAL) => {
                SPECIAL.parse(IN_node, IN_resUIData);
            },
        };

        IN_node.tag.giveBack(idkfa_nodeTag);

        // Render border
        let w_borderStart: Vector2 = (w_NodeUIData.position.0 - 1, w_NodeUIData.position.1 + 1);
        let w_borderEnd: Vector2 = (w_NodeUIData.position.0 + w_NodeUIData.size.0 as isize, w_NodeUIData.position.1 - w_NodeUIData.size.1 as isize);

        match IN_node.style.border {
            UI_border::none => {}
            UI_border::singleChar(CHAR) => {
                // Top
                self.drawLine((w_borderStart.0, w_borderStart.1), (w_borderEnd.0, w_borderStart.1), StyleSet{ ch: CHAR, fg: Color::White, bg: Color::Black });
                // Bottom
                self.drawLine((w_borderStart.0, w_borderEnd.1), (w_borderEnd.0, w_borderEnd.1), StyleSet{ ch: CHAR, fg: Color::White, bg: Color::Black });
                // Left
                self.drawLine((w_borderStart.0, w_borderStart.1), (w_borderStart.0, w_borderEnd.1), StyleSet{ ch: CHAR, fg: Color::White, bg: Color::Black });
                // Right
                self.drawLine((w_borderEnd.0, w_borderStart.1), (w_borderEnd.0, w_borderEnd.1), StyleSet{ ch: CHAR, fg: Color::White, bg: Color::Black });
            }
            UI_border::fancy =>{
                // Top
                self.drawLine((w_borderStart.0, w_borderStart.1), (w_borderEnd.0, w_borderStart.1), StyleSet{ ch: '═', fg: Color::White, bg: Color::Black });
                // Bottom
                self.drawLine((w_borderStart.0, w_borderEnd.1), (w_borderEnd.0, w_borderEnd.1), StyleSet{ ch: '═', fg: Color::White, bg: Color::Black });
                // Left
                self.drawLine((w_borderStart.0, w_borderStart.1), (w_borderStart.0, w_borderEnd.1), StyleSet{ ch: '║', fg: Color::White, bg: Color::Black });
                // Right
                self.drawLine((w_borderEnd.0, w_borderStart.1), (w_borderEnd.0, w_borderEnd.1), StyleSet{ ch: '║', fg: Color::White, bg: Color::Black });

                // Corners
                self.frameBuffer[(w_borderStart.0, w_borderStart.1)] = StyleSet{ ch: '╔', fg: Color::White, bg: Color::Black }; // TL
                self.frameBuffer[(w_borderEnd.0, w_borderStart.1)] = StyleSet{ ch: '╗', fg: Color::White, bg: Color::Black }; // TR
                self.frameBuffer[(w_borderStart.0, w_borderEnd.1)] = StyleSet{ ch: '╚', fg: Color::White, bg: Color::Black }; // BL
                self.frameBuffer[(w_borderEnd.0, w_borderEnd.1)] = StyleSet{ ch: '╝', fg: Color::White, bg: Color::Black }; // BR
            }
        }

        for NODE in IN_node.nodes.iter_mut(){
            self.renderUINode(NODE, &w_NodeUIData, IN_resUIData);
        }
    }

    pub fn drawLine(&mut self, IN_start: Vector2, IN_end: Vector2, IN_styleSet: StyleSet){
        // COPIED FROM OLD MANUFACTURE
        // And slightly adjusted

        
        // Init start values
        let mut w_startPos = IN_start;
        let mut w_endPos = IN_end;

        // Calc delta distance between points and check which is the main axis, then set the variables
        //          Delta X                          Delta Y
        if IN_start.0.abs_diff(IN_end.0) >= IN_start.1.abs_diff(IN_end.1){ // X Axis
            
            // Swap if needed
            if !(IN_start.0 < IN_end.0){
                // Wonder what's the cost of this
                std::mem::swap(&mut w_startPos, &mut w_endPos);
            }
            
            let mut w_yPos = w_startPos.1; // Set subaxis position
            let w_dir = if w_startPos.1 < w_endPos.1{1}else{-1}; // Check what way the line is going, set sign if needed -- Sign goes down

            // Finally iterate
            for XPOS in w_startPos.0..=w_endPos.0{
                self.frameBuffer[(XPOS, w_yPos)] = IN_styleSet;
    
                if w_yPos.abs_diff(w_endPos.1)*2 > XPOS.abs_diff(w_endPos.0){

                    w_yPos += w_dir
                }
            }
        }
        else{ // Y Axis

            // Swap if needed
            if !(IN_start.1 < IN_end.1){
                std::mem::swap(&mut w_startPos, &mut w_endPos);
            }

            let mut w_xPos = w_startPos.0; // Set subaxis position
            let w_dir = if w_startPos.0 < w_endPos.0{1}else{-1}; // Check what way the line is going, set sign if needed -- Sign goes left

            // Finally iterate
            for YPOS in w_startPos.1..=w_endPos.1{
                self.frameBuffer[(w_xPos, YPOS)] = IN_styleSet;
    
                if w_xPos.abs_diff(w_endPos.0)*2 > YPOS.abs_diff(w_endPos.1){
                    w_xPos += w_dir
                }
            }
        }
    }
}

pub struct sysData_Renderer<'a>{
    pub comp_Pos: ReadStorage<'a, comp_Pos>,
    pub comp_Sprite: ReadStorage<'a, comp_Sprite>,
    pub comp_ViewportCamera: ReadStorage<'a, comp_ViewportCamera>,
    pub comp_UIBox: WriteStorage<'a, comp_UIBox>,
    pub res_UIData: Fetch<'a, res_UIData>
}
impl<'a> gmSystemData<'a> for sysData_Renderer<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_Pos: IN_world.fetch(),
            comp_Sprite: IN_world.fetch(),
            comp_ViewportCamera: IN_world.fetch(),
            comp_UIBox: IN_world.fetchMut(),
            res_UIData: IN_world.fetchRes()
        }
    }
}