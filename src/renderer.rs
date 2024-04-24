use clearscreen::clear;
use crossterm::style::{Color, Stylize};
use std::{io::{stdout, Write}, time::Instant};

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

        let mut DEBUG_LOCK = SYS_debug.lock().unwrap();
        'INIT_debugStr: {
            DEBUG_LOCK.DATA_debugItems.insert(
                "#RENDER_frameTime".to_string(),
                IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_frameTime","", 255)
            );

            DEBUG_LOCK.DATA_debugItems.insert(
                "#RENDER_worldTime".to_string(),
                IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_worldTime","", 255)
            );

            DEBUG_LOCK.DATA_debugItems.insert(
                "#RENDER_convTime".to_string(),
                IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_convTime", "", 255)
            );

            DEBUG_LOCK.DATA_debugItems.insert(
                "#RENDER_borderTime".to_string(),
                IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_borderTime", "", 255)
            );

            DEBUG_LOCK.DATA_debugItems.insert(
                "#RENDER_textTime".to_string(),
                IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_textTime", "", 255)
            );

            DEBUG_LOCK.DATA_debugItems.insert(
                "#SSINIT_render".to_string(),
                IDDQD_textItem::newDebug(".DEBUG_sys/.SYS_ssInit/#SSINIT_render", "",  40)
            );
        }

        Self { 
            RENDER_bufferGrid: [TEMPLATE_wrCell::new(); system::SYS_REND_BUFFER_X * system::SYS_REND_BUFFER_Y]
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
    pub fn SYS_HANDLER_renderGame(&mut self) {
        let RENDER_start = Instant::now();
        let mut DEBUG_LOCK = SYS_debug.lock().unwrap();

        // Set cell for the player
        // TODO: Clean up
        self.r_util_setBufferCell(
            [system::SYS_REND_WORLD_X / 2 + 2, system::SYS_REND_WORLD_Y / 2 + 2], 
            'P', 
            SYS_data.lock().unwrap().DATA_player.p_color,
            false
        );

        {
            let loopStart = Instant::now();
            self.r_util_text();
            DEBUG_LOCK.DATA_debugItems.get_mut("#RENDER_textTime").unwrap().t_values = format!("{:?}", loopStart.elapsed())
        }
        
        {
            let loopStart = Instant::now();
            self.r_util_border([1, 1], [system::SYS_REND_WORLD_X + 1, system::SYS_REND_WORLD_Y + 1]);
            DEBUG_LOCK.DATA_debugItems.get_mut("#RENDER_borderTime").unwrap().t_values = format!("{:?}", loopStart.elapsed())
        }

        {
            let loopStart = Instant::now();
            self.r_util_world();
            DEBUG_LOCK.DATA_debugItems.get_mut("#RENDER_worldTime").unwrap().t_values = format!("{:?}", loopStart.elapsed())
        }

        // Convert buffer into string

        // I will leave this here cuz I'm proud of it but it's somehow SLOWER THAN NESTED `FOR` LOOPS
        // let RENDER_bufferstring = self.RENDER_bufferGrid
        //     .chunks(system::SYS_REND_BUFFER_X)
        //     .map(|ROW| ROW.iter().map(|CELL|
        //     CELL
        //         .c_char
        //         .with(CELL.c_colors[0])
        //         .on(CELL.c_colors[1])
        //         .to_string()
        //             ).collect::<Vec<String>>().concat()
        //     ).collect::<Vec<String>>().join(system::SYS_NEWLINE);

        // Clear and print new frame
        {
            let _ = clear();
            let loopStart = Instant::now();
            let mut STDOUT_LOCK = stdout().lock();
            for YPOS in 0..system::SYS_REND_BUFFER_Y - 1 {
                for XPOS in 0..system::SYS_REND_BUFFER_X - 1 {
                    let RENDER_cell = &self.RENDER_bufferGrid[XPOS + YPOS * system::SYS_REND_BUFFER_X];
                    write!(STDOUT_LOCK, "{}", RENDER_cell.c_char.with(RENDER_cell.c_colors[0]).on(RENDER_cell.c_colors[1])).unwrap();
                }
                write!(STDOUT_LOCK, "\r\n").unwrap()
            }
            stdout().flush().unwrap();
            self.RENDER_bufferGrid.fill(TEMPLATE_wrCell::new());
            DEBUG_LOCK.DATA_debugItems.get_mut("#RENDER_convTime").unwrap().t_values = format!("{:?}", loopStart.elapsed());
        }

        // DEBUG
        DEBUG_LOCK.DATA_debugItems.get_mut("#RENDER_frameTime").unwrap().t_values = format!("{:?}", RENDER_start.elapsed());
    }

    /// # Set buffer cell
    /// If the cell is already occupied it will not update the cell
    fn r_util_setBufferCell(&mut self, cPosition: system::coords, cChar: char, cColors: system::cellColors, forceOverwrite: bool) {
        let cellCoords = cPosition[0] + cPosition[1] * system::SYS_REND_BUFFER_X;
        if self.RENDER_bufferGrid[cellCoords].c_char != ' ' && !forceOverwrite{
            return;
        }
        self.RENDER_bufferGrid[cellCoords].CELL_update(Some(cChar), Some(cColors[0]), Some(cColors[1]))
    }

    /// # Render border
    /// Lets you render a border at specific coords with specific size
    /// 
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// It'll be rewritten/deleted in favor of Window system
    /// 
    /// TODO: make this cleaner
    fn r_util_border(&mut self, borderPos: system::coords, borderSizeInner: system::coords) {

        'BORDER_CORNERS:{
            // Corners first
            self.r_util_setBufferCell(
                borderPos,
                '╔', 
                [Color::White, Color::Black],
                false
            );
            self.r_util_setBufferCell(
                [borderPos[0] + borderSizeInner[0], borderPos[1]], 
                '╗', 
                [Color::White, Color::Black],
                false
            );
            self.r_util_setBufferCell(
                [borderPos[0], borderPos[1] + borderSizeInner[1]],
                '╚', 
                [Color::White, Color::Black],
                false
            );
            self.r_util_setBufferCell(
                [borderPos[0] + borderSizeInner[0], borderPos[1] + borderSizeInner[1]],
                '╝', 
                [Color::White, Color::Black],
                false
            );

        }

        // Top and bottom border
        for YPOS in [borderPos[1], borderPos[1] + borderSizeInner[1]] {
            for XPOS in borderPos[0] + 1..borderSizeInner[0] + 1 {
                self.r_util_setBufferCell(
                [XPOS, YPOS],
                '=', 
                [Color::White, Color::Black],
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
                [Color::White, Color::Black],
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
    fn r_util_text(&mut self) {
        let mut DATA_LOCK = SYS_data.lock().unwrap();
        for RTEXT in DATA_LOCK.DATA_textItems.iter_mut(){
            let mut RTEXT_charStartPosition = RTEXT.t_position.value();
            let mut RTEXT_charPosition = RTEXT_charStartPosition;
            'RENDER_textBlocks: for RTEXT_char in RTEXT.t_string.clone().chars() {
                if RTEXT_char == '\r'{
                    continue;
                }
                if RTEXT_char == '\n' {
                    RTEXT_charStartPosition[1] += 1;
                    RTEXT_charPosition = RTEXT_charStartPosition;
                    continue;
                }
                if RTEXT_charPosition[0] >= system::SYS_REND_BUFFER_X || RTEXT_charPosition[1] >= system::SYS_REND_BUFFER_Y {
                    break 'RENDER_textBlocks;
                }
                self.r_util_setBufferCell(RTEXT_charPosition, RTEXT_char, [Color::White, Color::Black], false);
                RTEXT_charPosition[0] += 1
            }
            if RTEXT.t_lifetime == 255{
                continue;
            }
            RTEXT.TEXT_tickdown();
        }

        DATA_LOCK.DATA_textItems.retain(|RTEXT| RTEXT.t_lifetime > 0)
    }

    /// # Render the world
    fn r_util_world(&mut self) {
        // First get vec of chunk references to not overload the system
        let DATA_LOCK = SYS_data.lock().unwrap();
        let r_workingChunkArray = DATA_LOCK.DATA_world.w_returnChunkArray(DATA_LOCK.DATA_player.p_chunk, [system::SYS_REND_CHUNK_X, system::SYS_REND_CHUNK_Y]);
        
        let r_workingBorderOffset = [
            (DATA_LOCK.DATA_player.p_pos[0] % system::SYS_CHUNK_X + system::SYS_REND_CHUNK_X / 2 * system::SYS_CHUNK_X) - system::SYS_REND_WORLD_X / 2,
            (DATA_LOCK.DATA_player.p_pos[1] % system::SYS_CHUNK_Y + system::SYS_REND_CHUNK_Y / 2 * system::SYS_CHUNK_Y) - system::SYS_REND_WORLD_Y / 2
        ];

        // I hate how much of a spaghett this is
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

    pub fn SYS_HANDLER_renderDebugStrs(&mut self){
        let mut DEBUG_LOCK = SYS_debug.lock().unwrap();
        let mut STDOUT_LOCK = stdout().lock();
        for DEBUGSTR in DEBUG_LOCK.DATA_debugItems.values_mut(){
            if &DEBUGSTR.t_string == "#MARK_FOR_DELETION"{
                continue;
            }

            write!(STDOUT_LOCK, "{} {}\r\n",
                &DEBUGSTR.t_string.clone().with(system::SYS_DEBUGCOLORS.0),
                &DEBUGSTR.t_values.clone().with(system::SYS_DEBUGCOLORS.1)).unwrap();

            DEBUGSTR.TEXT_tickdown()
        };
        STDOUT_LOCK.flush().unwrap();
    }
}



/// # Render Buffer Cell
/// 
/// Values:
/// 
/// * Character
/// * Colors for character and background
#[derive(Clone, Copy)]
pub struct TEMPLATE_wrCell {
    pub c_char: char,
    pub c_colors: system::cellColors
}
impl TEMPLATE_wrCell{
    pub fn new() -> Self{
        Self {c_char: ' ', c_colors: [system::SYS_DEFCOLORS.0, system::SYS_DEFCOLORS.1] }
    }
    /// Update cell colors
    /// It also refreshes the final string
    pub fn CELL_update(&mut self, IN_char: Option<char>, IN_FGColor: Option<Color>, IN_BGColor: Option<Color>){
        if IN_char.is_some(){self.c_char = IN_char.unwrap()}
        if IN_FGColor.is_some(){self.c_colors[0] = IN_FGColor.unwrap()}
        if IN_BGColor.is_some(){self.c_colors[1] = IN_BGColor.unwrap()}
    }
}
/// # Position in Render Buffer
/// A selection of common positions for useage
/// 
/// You can also use your custom position with `POS_custom`
pub enum RENDER_position{
    None,
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
impl RENDER_position {
    pub fn value(&self) -> system::coords{
        match *self {
            Self::None => [0, 0],
            Self::POS_middle => [system::SYS_REND_BUFFER_X / 2, system::SYS_REND_BUFFER_Y / 2],
            Self::POS_left => [0, system::SYS_REND_BUFFER_Y / 2],
            Self::POS_right => [system::SYS_REND_BUFFER_X - 1, system::SYS_REND_BUFFER_Y / 2],
            Self::POS_top => [system::SYS_REND_BUFFER_X / 2, 0],
            Self::POS_bottom => [system::SYS_REND_BUFFER_X / 2, system::SYS_REND_BUFFER_Y - 1],
            Self::POS_TL => [0, 0],
            Self::POS_TR => [system::SYS_REND_BUFFER_X - 1, 0],
            Self::POS_BL => [0, system::SYS_REND_BUFFER_Y - 1],
            Self::POS_BR => [system::SYS_REND_BUFFER_X - 1, system::SYS_REND_BUFFER_Y - 1],
            Self::POS_custom(POS) => POS
        }
    }
}

/// # Render Buffer
/// 
/// Holds full size cells
struct RENDER_buffer{
    pub BUFFER_grid: Vec<TEMPLATE_wrCell>
}
impl RENDER_buffer{
    pub fn new(IN_size: system::coords) -> Self{
        Self { BUFFER_grid: vec![TEMPLATE_wrCell::new(); IN_size[0] * IN_size[1]]}
    }
    pub fn reset(&mut self){
        self.BUFFER_grid.fill(TEMPLATE_wrCell::new())
    }
}
impl Index<system::coords> for RENDER_buffer{
    type Output = TEMPLATE_wrCell;
    fn index(&self, index: system::coords) -> &Self::Output {
        &self.BUFFER_grid[index[0] + index[1] * system::SYS_REND_BUFFER_X]
    }
}

/// # Cheap render buffer
/// 
/// Holds *references* to cells in it's palette
/// 
/// TODO: actually make something use it
struct RENDER_cheapBuffer{
    pub CBUFFER_grid: Vec<&'static TEMPLATE_wrCell>,
    pub CBUFFER_palette: Vec<TEMPLATE_wrCell>
}
impl RENDER_cheapBuffer{
    const idkfa_cell: renderer::TEMPLATE_wrCell = TEMPLATE_wrCell::new();
    pub fn new(IN_size: system::coords) -> Self{
        Self{
            CBUFFER_grid: vec![&Self::idkfa_cell; IN_size[0] * IN_size[1]],
            CBUFFER_palette: Vec::new()
        }
    }
    pub fn reset(&mut self){
        self.CBUFFER_grid.fill(&Self::idkfa_cell)
    }
}
impl Index<system::coords> for RENDER_cheapBuffer{
    type Output = TEMPLATE_wrCell;
    fn index(&self, index: system::coords) -> &Self::Output {
        &self.CBUFFER_grid[index[0] + index[1] * system::SYS_REND_BUFFER_X]
    }
}