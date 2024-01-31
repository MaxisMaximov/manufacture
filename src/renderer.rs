use clearscreen::clear;
use crossterm::style::{Color, Stylize};
use std::time::Instant;

use crate::index;
use crate::system;

/// # Text renderer
/// ## Disclaimer
/// It will be replaced should I move onto Pixel renderer
/// 
/// Although hopefully the new one will be <<compatible
pub struct SYS_RENDERER{
    RENDER_bufferGrid: [index::TEMPLATE_wrCell; system::SYS_REND_BUFFER_X * system::SYS_REND_BUFFER_Y],
    RENDER_text: Vec<index::RENDER_textItem>,
    RENDER_debug: String
}
impl SYS_RENDERER{
    pub fn new() -> Self{
        SYS_RENDERER { 
            RENDER_bufferGrid: [index::TEMPLATE_wrCell::new(); system::SYS_REND_BUFFER_X * system::SYS_REND_BUFFER_Y],
            RENDER_text: vec![],
            RENDER_debug: String::new()
        }
    }

    /// # Push debug string for render
    /// 
    /// ### Disclaimer?
    /// If you push too many debug strings the terminal might get aneurysm
    pub fn r_pushDebugStr(&mut self, INr_string: &str){
        self.RENDER_debug.push_str(INr_string)
    }

    /// # Push text for rendering
    /// 
    /// # DO NOT RELY ON THIS
    /// Will also be rewritten in favor of Window system
    pub fn r_pushText(&mut self, INr_textItem: index::RENDER_textItem){
        self.RENDER_text.push(INr_textItem)
    }

    /// # Render game
    /// The renderer handles in this order:
    /// - Clear buffer
    /// - Render Player
    /// - Render text
    /// - Render any borders you've inputted << DO NOT RELY
    /// - Render world
    /// - Convert buffer into printable string
    /// - Display frame
    /// - Display debug string
    pub fn SYS_HANDLER_renderGame(&mut self, INr_player: &index::TEMPLATE_player, INr_world: &index::TEMPLATE_world) {
        let RENDER_start = Instant::now();

        // Set cell for the player
        // TODO: Clean up
        self.r_util_setBufferCell(
            self.r_util_calcPos(
                [system::SYS_REND_WORLD_X / 2, system::SYS_REND_WORLD_Y / 2], 
                [2,2]
            ), 
            'P', 
            INr_player.p_colorChar, 
            INr_player.p_colorBg);

        self.r_util_text();

        self.r_util_border([1, 1], [system::SYS_REND_WORLD_X + 1, system::SYS_REND_WORLD_Y + 1]);

        self.r_util_border([1, 20], [36, 6]);

        self.r_util_world(INr_world, INr_player);

        // Convert buffer into string
        let mut RENDER_bufferstring = String::new();
        for YPOS in 0..system::SYS_REND_BUFFER_Y - 1 {
            for XPOS in 0..system::SYS_REND_BUFFER_X - 1 {
                let RENDER_cell = self.RENDER_bufferGrid[XPOS + YPOS * system::SYS_REND_BUFFER_X];
                RENDER_bufferstring.push_str(
                    &RENDER_cell
                        .c_char
                        .with(RENDER_cell.c_colChr)
                        .on(RENDER_cell.c_colBg)
                        .to_string(),
                )
            }
            RENDER_bufferstring.push('\n')
        }

        // DEBUG
        let r_frameTime =  RENDER_start.elapsed();
        self.RENDER_debug.push_str(&format!(
            "Finished frame rendering in {:?}\nApproximate FPS: {}",
            r_frameTime,
            1000 / r_frameTime.as_millis()
        ));

        // Clear and print new frame
        clear();
        println!("{}", RENDER_bufferstring);

        println!("{}", self.RENDER_debug);

        // Reset buffers
        self.RENDER_bufferGrid.fill(index::TEMPLATE_wrCell::new());
        self.RENDER_debug.clear();
    }

    /// # Set buffer cell
    /// If the cell is already occupied it will not update the cell
    fn r_util_setBufferCell(&mut self, cPosition: usize, cChar: char, cColChr: Color, cColBg: Color,) {
        if self.RENDER_bufferGrid[cPosition].c_char != ' '{
            return;
        }
        self.RENDER_bufferGrid[cPosition] = index::TEMPLATE_wrCell {
            c_char: cChar,
            c_colChr: cColChr,
            c_colBg: cColBg,
        }
    }

    /// # Calculate Buffer position with offset
    /// ## Disclaimer
    /// It will be moved elsewhere once other systems will rely on it
    fn r_util_calcPos(&self, localPos: [usize; 2], offsetPos: [usize; 2]) -> usize {
        return ((localPos[0] + offsetPos[0]) + (localPos[1] + offsetPos[1]) * system::SYS_REND_BUFFER_X);
    }

    /// # Render border
    /// Lets you render a border at specific coords with specific size
    /// 
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// It'll be rewritten/deleted in favor of Window system
    /// 
    /// TODO: make this cleaner
    fn r_util_border(&mut self, borderPos: [usize; 2], borderSizeInner: [usize; 2]) {
        // Corners first

        self.r_util_setBufferCell(
            self.r_util_calcPos(
                    borderPos, 
                    [0, 0]
                    ),
            '╔', 
            Color::White, 
            Color::Black
        );
        self.r_util_setBufferCell(
            self.r_util_calcPos(
                [borderPos[0] + borderSizeInner[0], borderPos[1]], 
                [0, 0]
                ), 
            '╗', 
            Color::White, 
            Color::Black
        );
        self.r_util_setBufferCell(
            self.r_util_calcPos(
                [borderPos[0], borderPos[1] + borderSizeInner[1]], 
                [0, 0]
                ), 
            '╚', 
            Color::White, 
            Color::Black
        );
        self.r_util_setBufferCell(
            self.r_util_calcPos(
                [borderPos[0] + borderSizeInner[0], borderPos[1] + borderSizeInner[1],], 
                [0, 0]
                ), 
            '╝', 
            Color::White, 
            Color::Black
        );

        // Top and bottom border
        for YPOS in [borderPos[1], borderPos[1] + borderSizeInner[1]] {
            for XPOS in borderPos[0] + 1..borderSizeInner[0] + 1 {
                self.r_util_setBufferCell(
                    self.r_util_calcPos(
                        [XPOS, YPOS], 
                        [0, 0]
                    ), 
                '=', 
                Color::White,
                Color::Black)
            }
        }
        // Left and right border
        for XPOS in [borderPos[0], borderPos[0] + borderSizeInner[0]] {
            for YPOS in borderPos[1] + 1..borderPos[1] + borderSizeInner[1] {
                self.r_util_setBufferCell(
                    self.r_util_calcPos(
                        [XPOS, YPOS],
                        [0, 0]
                ),
                '‖',
                Color::White,
                Color::Black)
            }
        }
    }


    /// # Render text
    /// Renders text in `RENDER_text` vector
    /// 
    /// # DO NOT RELY ON THIS
    /// It'll be most likely removed in favor of Window system
    fn r_util_text(&mut self) {
        for RTEXT_index in 0..self.RENDER_text.len() {
            let mut RTEXT_charStartIndex = self.r_util_calcPos(
                [
                    self.RENDER_text[RTEXT_index].t_position[0],
                    self.RENDER_text[RTEXT_index].t_position[1],
                ],
                [0, 0],
            );
            let mut RTEXT_charIndex = RTEXT_charStartIndex;
            'RENDER_textBlocks: for RTEXT_char in self.RENDER_text[RTEXT_index].t_text.clone().chars() {
                if RTEXT_char == '\n' {
                    RTEXT_charIndex = RTEXT_charStartIndex + system::SYS_REND_BUFFER_X;
                    RTEXT_charStartIndex = RTEXT_charIndex;
                    continue;
                }
                if RTEXT_charIndex > self.RENDER_bufferGrid.len()-1 {
                    self.RENDER_debug.push_str(&format!(
                        "STRING ERROR: Out of Bounds\nString: --{}--\nLocation: X: {} Y: {}\n",
                        self.RENDER_text[RTEXT_index].t_text,
                        self.RENDER_text[RTEXT_index].t_position[0],
                        self.RENDER_text[RTEXT_index].t_position[1]
                    ));
                    break 'RENDER_textBlocks;
                }
                self.r_util_setBufferCell(RTEXT_charIndex, RTEXT_char, Color::White, Color::Black);
                RTEXT_charIndex += 1
            }
            if self.RENDER_text[RTEXT_index].t_lifetime == 255{
                continue;
            }
            self.RENDER_text[RTEXT_index].t_lifetime -= 1;
        }

        self.RENDER_text.retain(|RTEXT| RTEXT.t_lifetime > 0)
    }

    /// # Render the world
    fn r_util_world(&mut self, INr_world: &index::TEMPLATE_world, INr_player: &index::TEMPLATE_player) {
        let r_workingChunkArray = INr_world.w_returnChunkArray([INr_player.p_chunkX, INr_player. p_chunkY], [system::SYS_REND_CHUNK_X, system::SYS_REND_CHUNK_Y]);
        let r_workingBorderOffset = [
            (INr_player.p_x % system::SYS_CHUNK_X + system::SYS_REND_CHUNK_X / 2 * system::SYS_CHUNK_X) - system::SYS_REND_WORLD_X / 2,
            (INr_player.p_y % system::SYS_CHUNK_Y + system::SYS_REND_CHUNK_Y / 2 * system::SYS_CHUNK_Y) - system::SYS_REND_WORLD_Y / 2
        ];

        for YPOS in 0..system::SYS_REND_WORLD_Y{
            for XPOS in 0..system::SYS_REND_WORLD_X{
                let r_workingChunkCell = &r_workingChunkArray[
                    (r_workingBorderOffset[0] + XPOS)/system::SYS_CHUNK_X + 
                    (r_workingBorderOffset[1] + YPOS) /system::SYS_CHUNK_Y * system::SYS_REND_CHUNK_Y
                    ].ch_cells[
                        (r_workingBorderOffset[0] + XPOS) % system::SYS_CHUNK_X + 
                        (r_workingBorderOffset[1] + YPOS) % system::SYS_CHUNK_Y * system::SYS_CHUNK_Y
                        ];
                self.r_util_setBufferCell(self.r_util_calcPos([XPOS, YPOS], [2,2]), 
                    r_workingChunkCell.c_char, 
                    r_workingChunkCell.c_colChr, 
                    r_workingChunkCell.c_colBg
                );
            }
        }
    }
}