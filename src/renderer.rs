use clearscreen::clear;
use crossterm::style::{Color, Stylize};
use std::time::Instant;

use crate::*;

/// # Text renderer
/// ## Disclaimer
/// It will be replaced should I move onto Pixel renderer
/// 
/// Although hopefully the new one will be <<compatible
pub struct SYS_RENDERER{
    RENDER_bufferGrid: [TEMPLATE_wrCell; system::SYS_REND_BUFFER_X * system::SYS_REND_BUFFER_Y],
}
impl SYS_RENDERER{
    pub fn new() -> Self{
        SYS_RENDERER { 
            RENDER_bufferGrid: [TEMPLATE_wrCell::new(); system::SYS_REND_BUFFER_X * system::SYS_REND_BUFFER_Y],
        }
    }

    /// # Render game
    /// The renderer handles in this order:
    /// - Render Player
    /// - Render text
    /// - Render any borders you've inputted << DO NOT RELY
    /// - Render world
    /// - Convert buffer into printable string
    /// - Clear screen for next frame
    /// - Display frame
    /// - Display debug string
    pub fn SYS_HANDLER_renderGame(&mut self, INr_data: &mut DATA_master) {
        let RENDER_start = Instant::now();

        // Set cell for the player
        // TODO: Clean up
        self.r_util_setBufferCell(
            [system::SYS_REND_WORLD_X / 2 + 2, system::SYS_REND_WORLD_Y / 2 + 2], 
            'P', 
            INr_data.DATA_player.p_color,
            false
        );

        self.r_util_text(INr_data);

        self.r_util_border([1, 1], [system::SYS_REND_WORLD_X + 1, system::SYS_REND_WORLD_Y + 1]);

        self.r_util_world(INr_data);

        // Convert buffer into string
        let mut RENDER_bufferstring = String::new();
        for YPOS in 0..system::SYS_REND_BUFFER_Y - 1 {
            for XPOS in 0..system::SYS_REND_BUFFER_X - 1 {
                let RENDER_cell = self.RENDER_bufferGrid[XPOS + YPOS * system::SYS_REND_BUFFER_X];
                RENDER_bufferstring.push_str(
                    &RENDER_cell
                        .c_char
                        .with(RENDER_cell.c_colors[0])
                        .on(RENDER_cell.c_colors[1])
                        .to_string(),
                )
            }
            RENDER_bufferstring.push_str(system::SYS_NEWLINE)
        }

        // DEBUG
        let r_frameTime = RENDER_start.elapsed();
        INr_data.DATA_debug.push_str(&format!(
            "Finished frame rendering in {:?}{NEW}Approximate FPS: {}",
            r_frameTime,
            1000000 / r_frameTime.as_micros(),
            NEW = system::SYS_NEWLINE
        ));

        // Clear and print new frame
        clear();
        println!("{}", RENDER_bufferstring);
        // Reset buffers
        self.RENDER_bufferGrid.fill(TEMPLATE_wrCell::new());
    }

    /// # Set buffer cell
    /// If the cell is already occupied it will not update the cell
    fn r_util_setBufferCell(&mut self, cPosition: system::coords, cChar: char, cColors: system::cellColors, forceOverwrite: bool) {
        let cellCoords = cPosition[0] + cPosition[1] * system::SYS_REND_BUFFER_X;
        if self.RENDER_bufferGrid[cellCoords].c_char != ' ' && !forceOverwrite{
            return;
        }
        self.RENDER_bufferGrid[cellCoords] = TEMPLATE_wrCell {
            c_char: cChar,
            c_colors: cColors
        }
    }

    /// # Calculate Buffer position with offset
    /// ## Disclaimer
    /// It will be moved elsewhere once other systems will rely on it
    fn r_util_calcPos(&self, localPos: system::coords, offsetPos: system::coords) -> usize {
        return ((localPos[0] + offsetPos[0]) + (localPos[1] + offsetPos[1]) * system::SYS_REND_BUFFER_X);
    }

    /// # Render border
    /// Lets you render a border at specific coords with specific size
    /// 
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// It'll be rewritten/deleted in favor of Window system
    /// 
    /// TODO: make this cleaner
    fn r_util_border(&mut self, borderPos: system::coords, borderSizeInner: system::coords) {
        // Corners first

        self.r_util_setBufferCell(
            borderPos,
            '╔', 
            [Color::White, 
            Color::Black],
            false
        );
        self.r_util_setBufferCell(
            [borderPos[0] + borderSizeInner[0], borderPos[1]], 
            '╗', 
            [Color::White, 
            Color::Black],
            false
        );
        self.r_util_setBufferCell(
            [borderPos[0], borderPos[1] + borderSizeInner[1]],
            '╚', 
            [Color::White, 
            Color::Black],
            false
        );
        self.r_util_setBufferCell(
            [borderPos[0] + borderSizeInner[0], borderPos[1] + borderSizeInner[1]],
            '╝', 
            [Color::White, 
            Color::Black],
            false
        );

        // Top and bottom border
        for YPOS in [borderPos[1], borderPos[1] + borderSizeInner[1]] {
            for XPOS in borderPos[0] + 1..borderSizeInner[0] + 1 {
                self.r_util_setBufferCell(
                [XPOS, YPOS],
                '=', 
                [Color::White,
                Color::Black],
                false
                )
            }
        }
        // Left and right border
        for XPOS in [borderPos[0], borderPos[0] + borderSizeInner[0]] {
            for YPOS in borderPos[1] + 1..borderPos[1] + borderSizeInner[1] {
                self.r_util_setBufferCell(
                [XPOS, YPOS],
                '‖',
                [Color::White,
                Color::Black],
                false
                )
            }
        }
    }


    /// # Render text
    /// Renders text in `RENDER_text` vector
    /// 
    /// # DO NOT RELY ON THIS
    /// It'll be most likely removed in favor of Window system
    fn r_util_text(&mut self, INr_data: &mut DATA_master) {
        for RTEXT_index in 0..INr_data.DATA_textItems.len() {
            let mut RTEXT_charStartPosition =
                match INr_data.DATA_textItems[RTEXT_index].t_position {
                    RENDER_position::POS_middle => [system::SYS_REND_BUFFER_X / 2, system::SYS_REND_BUFFER_Y / 2],
                    RENDER_position::POS_left => [0, system::SYS_REND_BUFFER_Y / 2],
                    RENDER_position::POS_right => [system::SYS_REND_BUFFER_X - 1, system::SYS_REND_BUFFER_Y / 2],
                    RENDER_position::POS_top => [system::SYS_REND_BUFFER_X / 2, 0],
                    RENDER_position::POS_bottom => [system::SYS_REND_BUFFER_X / 2, system::SYS_REND_BUFFER_Y - 1],
                    RENDER_position::POS_TL => [0, 0],
                    RENDER_position::POS_TR => [system::SYS_REND_BUFFER_X - 1, 0],
                    RENDER_position::POS_BL => [0, system::SYS_REND_BUFFER_Y - 1],
                    RENDER_position::POS_BR => [system::SYS_REND_BUFFER_X - 1, system::SYS_REND_BUFFER_Y - 1],
                    RENDER_position::POS_custom(POS) => POS
                };
            let mut RTEXT_charPosition = RTEXT_charStartPosition;
            'RENDER_textBlocks: for RTEXT_char in INr_data.DATA_textItems[RTEXT_index].t_text.clone().chars() {
                if RTEXT_char == '\r'{
                    continue;
                }
                if RTEXT_char == '\n' {
                    RTEXT_charStartPosition[1] += 1;
                    RTEXT_charPosition = RTEXT_charStartPosition;
                    continue;
                }
                if RTEXT_charPosition[0] > system::SYS_REND_BUFFER_X - 1 || RTEXT_charPosition[1] > system::SYS_REND_BUFFER_Y - 1 {
                    INr_data.DATA_pushDebugStr(format!(
                        "STRING ERROR: Out of Bounds{NEW}String: --{}--{NEW}Location: X: {} Y: {}",
                        INr_data.DATA_textItems[RTEXT_index].t_text,
                        RTEXT_charPosition[0],
                        RTEXT_charPosition[1],
                        NEW = system::SYS_NEWLINE
                    ));
                    break 'RENDER_textBlocks;
                }
                self.r_util_setBufferCell(RTEXT_charPosition, RTEXT_char, [Color::White, Color::Black], false);
                RTEXT_charPosition[0] += 1
            }
            if INr_data.DATA_textItems[RTEXT_index].t_lifetime == 255{
                continue;
            }
            INr_data.DATA_textItems[RTEXT_index].t_lifetime -= 1;
        }

        INr_data.DATA_textItems.retain(|RTEXT| RTEXT.t_lifetime > 0)
    }

    /// # Render the world
    fn r_util_world(&mut self, INr_data: &mut DATA_master) {
        let r_workingChunkArray = INr_data.DATA_world.w_returnChunkArray(INr_data.DATA_player.p_chunk, [system::SYS_REND_CHUNK_X, system::SYS_REND_CHUNK_Y]);
        let r_workingBorderOffset = [
            (INr_data.DATA_player.p_pos[0] % system::SYS_CHUNK_X + system::SYS_REND_CHUNK_X / 2 * system::SYS_CHUNK_X) - system::SYS_REND_WORLD_X / 2,
            (INr_data.DATA_player.p_pos[1] % system::SYS_CHUNK_Y + system::SYS_REND_CHUNK_Y / 2 * system::SYS_CHUNK_Y) - system::SYS_REND_WORLD_Y / 2
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
                self.r_util_setBufferCell([XPOS + 2, YPOS + 2], 
                    r_workingChunkCell.c_char, 
                    r_workingChunkCell.c_color,
                    false
                );
            }
        }
    }
}


/// # "Textbox" struct
/// Lets you paste a text somewhere in the game screen
/// 
/// # DO NOT RELY ON THIS
/// It'll be replaced in favor of Window system
/// 
/// # Warning
/// The Renderer doesn't check if the text overflows the X position yet, only if it's outside the buffer
/// 
/// So be careful where and what you write
pub struct RENDER_textItem{
    pub t_position: RENDER_position,
    pub t_text: String,
    pub t_lifetime: u16
}


/// # Render Buffer Cell
/// 
/// Values:
/// 
/// * Character
/// * Colors for character and background
pub struct TEMPLATE_wrCell {
    pub c_char: char,
    pub c_colors: system::cellColors
}
impl TEMPLATE_wrCell{
    pub fn new() -> Self{
        TEMPLATE_wrCell { c_char: ' ', c_colors: [Color::White, Color::Black] }
    }
}
impl Copy for TEMPLATE_wrCell {}
impl Clone for TEMPLATE_wrCell {
    fn clone(&self) -> Self {
        TEMPLATE_wrCell {
            c_char: self.c_char,
            c_colors: self.c_colors
        }
    }
}

pub enum RENDER_position{
    POS_middle,
    POS_right,
    POS_left,
    POS_top,
    POS_bottom,
    POS_TL,
    POS_TR,
    POS_BL,
    POS_BR,
    POS_custom(system::coords)
}