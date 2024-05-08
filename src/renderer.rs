use crossterm::{
    cursor::MoveTo,
    style::{Color, Stylize},
    terminal::{BeginSynchronizedUpdate, Clear, EndSynchronizedUpdate},
    ExecutableCommand,
};
use std::{
    io::{stdout, Write as ioWrite},
    ops::{Index, IndexMut},
    time::Instant,
};

use crate::*;

/// # Text renderer
/// ## Disclaimer
/// It will be replaced should I move onto Pixel renderer
///
/// Although hopefully the new one will be <<compatible

static RENDER_mainBuffer: Lazy<Mutex<RENDER_buffer>> = Lazy::new(|| {
    Mutex::new(RENDER_buffer::new([
        system::SYS_REND_BUFFER_X,
        system::SYS_REND_BUFFER_Y,
    ]))
});

pub fn new() {
    let mut DEBUG_LOCK = SYS_debug.lock().unwrap();
    'INIT_debugStr: {
        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_frameTime".to_string(),
            IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_frameTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_worldTime".to_string(),
            IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_worldTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_convTime".to_string(),
            IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_convTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_borderTime".to_string(),
            IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_borderTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_textTime".to_string(),
            IDDQD_textItem::newDebug(".DEBUG_render/#RENDER_textTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#SSINIT_render".to_string(),
            IDDQD_textItem::newDebug(".DEBUG_sys/.SYS_ssInit/#SSINIT_render", "", 40),
        );
    }
}

/// # Render game
pub fn SYS_HANDLER_renderGame() {
    let RENDER_start = Instant::now();
    let mut DEBUG_LOCK = SYS_debug.lock().unwrap();

    'RENDER_renderWorld: {
        let loopStart = Instant::now();

        self::r_util_world();

        DEBUG_LOCK
        .DATA_debugItems
        .get_mut("#RENDER_worldTime")
        .unwrap()
        .t_values = format!("{:?}", loopStart.elapsed())
    }

    // Set cell for the player
    // TODO: Clean up
    'RENDER_playerSet: {
        let DATA_LOCK = SYS_data.lock().unwrap();

        RENDER_mainBuffer.lock().unwrap()[[
            (system::SYS_REND_WORLD_X + 2),
            (system::SYS_REND_WORLD_Y + 2),
        ]] = TEMPLATE_wrCell {
            c_char: 'P',
            c_colors: DATA_LOCK.DATA_player.p_color,
        };
    }

    'RENDER_renderText: {
        let loopStart = Instant::now();

        self::r_util_text();

        DEBUG_LOCK
        .DATA_debugItems
        .get_mut("#RENDER_textTime")
        .unwrap()
        .t_values = format!("{:?}", loopStart.elapsed())
    }

    'RENDER_renderWorldBorder: {
        let loopStart = Instant::now();

        self::r_util_border(
            [1, 1],
            [system::SYS_REND_WORLD_X * 2 + 1, system::SYS_REND_WORLD_Y * 2 + 1],
        );

        DEBUG_LOCK
        .DATA_debugItems
        .get_mut("#RENDER_borderTime")
        .unwrap()
        .t_values = format!("{:?}", loopStart.elapsed())
    }
    
    

    // LINE DRAWER TEST
    // self::r_util_drawBufferLineAngle([4, 4], [2,0], '#', [Color::Cyan, Color::Cyan]);
    // self::r_util_drawBufferLineAngle([4, 4], [0,2], '#', [Color::Cyan, Color::Cyan]);
    // self::r_util_drawBufferLineAngle([4, 4], [7,0], '#', [Color::Cyan, Color::Cyan]);
    // self::r_util_drawBufferLineAngle([4, 4], [9,2], '#', [Color::Cyan, Color::Cyan]);
    // self::r_util_drawBufferLineAngle([4, 4], [7,9], '#', [Color::Cyan, Color::Cyan]);
    // self::r_util_drawBufferLineAngle([4, 4], [9,7], '#', [Color::Cyan, Color::Cyan]);
    // self::r_util_drawBufferLineAngle([4, 4], [2,9], '#', [Color::Cyan, Color::Cyan]);
    // self::r_util_drawBufferLineAngle([4, 4], [0,7], '#', [Color::Cyan, Color::Cyan]);
    
    // Print frame
    'RENDER_printFrame: {
        let loopStart = Instant::now();
        let mut STDOUT_LOCK = stdout().lock();
        let mut BUFFER_LOCK = RENDER_mainBuffer.lock().unwrap();
        
        // Start sync
        let _ = STDOUT_LOCK.execute(BeginSynchronizedUpdate);
        
        // Clear the screen
        let _ = execute!(
            STDOUT_LOCK,
            Clear(crossterm::terminal::ClearType::All),
            MoveTo(0, 0)
        );
        
        // Convert buffer to string
        for YPOS in 0..system::SYS_REND_BUFFER_Y {
            for XPOS in 0..system::SYS_REND_BUFFER_X {
                let RENDER_cell = BUFFER_LOCK[[XPOS, YPOS]];
                let _ = write!(
                    STDOUT_LOCK,
                    "{}",
                    RENDER_cell
                        .c_char
                        .with(RENDER_cell.c_colors[0])
                        .on(RENDER_cell.c_colors[1])
                );
            }
            write!(STDOUT_LOCK, "\r\n").unwrap()
        }

        // End sync and push the frame
        let _ = STDOUT_LOCK.execute(EndSynchronizedUpdate);

        // Reset the buffer
        BUFFER_LOCK.reset();

        // And log the time for conversion
        DEBUG_LOCK
            .DATA_debugItems
            .get_mut("#RENDER_convTime")
            .unwrap()
            .t_values = format!("{:?}", loopStart.elapsed());
    }

    // Log how long the entire process took
    DEBUG_LOCK
        .DATA_debugItems
        .get_mut("#RENDER_frameTime")
        .unwrap()
        .t_values = format!("{:?}", RENDER_start.elapsed());
}

#[allow(dead_code)]
fn r_util_drawBufferLineAngle(
    IN_pos_A: system::coords,
    IN_pos_B: system::coords,
    IN_char: char,
    IN_colors: system::cellColors,
) {
    let mut BUFFER_LOCK = self::RENDER_mainBuffer.lock().unwrap();

    // Init start values
    let w_startPos: system::coords;
    let w_endPos: system::coords;

    // Calc delta distance between points
    let w_deltaX = IN_pos_A[0].abs_diff(IN_pos_B[0]);
    let w_deltaY = IN_pos_A[1].abs_diff(IN_pos_B[1]);

    // Check which is the main axis
    if w_deltaX >= w_deltaY{ // X Axis
        
        // Check and set position
        'CHECK_pos:{
            if IN_pos_A[0] < IN_pos_B[0]{
                w_startPos = IN_pos_A;
                w_endPos = IN_pos_B
            }
            else{
                w_startPos = IN_pos_B;
                w_endPos = IN_pos_A
            }
        }
        
        let mut w_curY = w_startPos[1]; // Set subaxis position
        let w_sign = if w_startPos[1] < w_endPos[1]{false} else{true}; // Check what way the line is going, set sign if needed

        // Iterate
        for XPOS in w_startPos[0]..=w_endPos[0]{
            BUFFER_LOCK[[XPOS, w_curY]] = TEMPLATE_wrCell{ c_char: IN_char, c_colors: IN_colors };

            // Idk who made this equation but why does it work
            // SubaxisDelta * 2 > SuperaxisDelta
            if w_curY.abs_diff(w_endPos[1])*2 > XPOS.abs_diff(w_endPos[0]){
                // If sign is enabled that means it goes down
                if w_sign{w_curY -= 1}
                else{w_curY += 1}
            }
        }
    }
    else{ // Y Axis

        // Check  and set position
        'CHECK_pos:{
            if IN_pos_A[1] < IN_pos_B[1]{
                w_startPos = IN_pos_A;
                w_endPos = IN_pos_B
            }
            else{
                w_startPos = IN_pos_B;
                w_endPos = IN_pos_A
            }
        }

        let mut w_curX = w_startPos[0]; // Set subaxis position
        let w_sign = if w_startPos[0] < w_endPos[0]{false} else{true}; // Check what way the line is going, set sign if needed

        // Iterate
        for YPOS in w_startPos[1]..=w_endPos[1]{
            BUFFER_LOCK[[w_curX, YPOS]] = TEMPLATE_wrCell{ c_char: IN_char, c_colors: IN_colors };

            // Idk who made this equation but why does it work
            // SubaxisDelta * 2 > SuperaxisDelta
            if w_curX.abs_diff(w_endPos[0])*2 > YPOS.abs_diff(w_endPos[1]){
                // If sign is enabled that means it goes left
                if w_sign {w_curX -= 1}
                else{w_curX += 1}
            }
        }
    }
}

/// # Render border
/// Lets you render a border at specific coords with specific size
fn r_util_border(borderPos: system::coords, borderSizeInner: system::coords) {
    
    let w_corners = (
        borderPos, // TL
        [borderPos[0] + borderSizeInner[0], borderPos[1]], // TR
        [borderPos[0], borderPos[1] + borderSizeInner[1]], // BL
        [borderPos[0] + borderSizeInner[0], borderPos[1] + borderSizeInner[1]] // BR
    );
    
    self::r_util_drawBufferLineAngle(w_corners.0, w_corners.1, '=', [Color::White, Color::Black]);
    self::r_util_drawBufferLineAngle(w_corners.2, w_corners.3, '=', [Color::White, Color::Black]);
    self::r_util_drawBufferLineAngle(w_corners.0, w_corners.2, '‖', [Color::White, Color::Black]);
    self::r_util_drawBufferLineAngle(w_corners.1, w_corners.3, '‖', [Color::White, Color::Black]);
    
    let mut BUFFER_LOCK = self::RENDER_mainBuffer.lock().unwrap();
    // Corners at end cuz it's easier to set them
    'BORDER_CORNERS: {
        'TOP_LEFT: {
            BUFFER_LOCK[w_corners.0] = TEMPLATE_wrCell {
                c_char: '╔',
                c_colors: [Color::White, Color::Black],
            }
        }
        'TOP_RIGHT: {
            BUFFER_LOCK[w_corners.1] = TEMPLATE_wrCell {
                c_char: '╗',
                c_colors: [Color::White, Color::Black],
            }
        }
        'BOTTOM_LEFT: {
            BUFFER_LOCK[w_corners.2] = TEMPLATE_wrCell {
                c_char: '╚',
                c_colors: [Color::White, Color::Black],
            }
        }
        'BOTTOM_RIGHT: {
            BUFFER_LOCK[w_corners.3] = TEMPLATE_wrCell {
                c_char: '╝',
                c_colors: [Color::White, Color::Black],
            }
        }
    }
}

/// # Render text
/// Renders text in `RENDER_text` vector
///
/// # DO NOT RELY ON THIS
/// It'll be most likely removed in favor of Window system
fn r_util_text() {
    let mut DATA_LOCK = SYS_data.lock().unwrap();
    let mut BUFFER_LOCK = RENDER_mainBuffer.lock().unwrap();

    let mut w_skipToNewline = false;

    for RTEXT in DATA_LOCK.DATA_textItems.iter_mut() {

        let mut RTEXT_charStartPosition = RTEXT.t_position.value();
        let mut RTEXT_charPosition = RTEXT_charStartPosition;
        'RENDER_textBlocks: for RTEXT_char in RTEXT.t_string.clone().chars() {

            if RTEXT_char == '\r' {
                continue;
            }
            if RTEXT_char == '\n' {
                w_skipToNewline = false;
                RTEXT_charStartPosition[1] += 1;
                RTEXT_charPosition = RTEXT_charStartPosition;
                continue;
            }

            if w_skipToNewline{continue}

            // If X exceeds mark the skip
            if RTEXT_charPosition[0] >= system::SYS_REND_BUFFER_X{
                w_skipToNewline = true;
                continue;
            }

            // If Y exceeds there's no hope for the string
            if RTEXT_charPosition[1] >= system::SYS_REND_BUFFER_Y
            {
                break 'RENDER_textBlocks;
            }

            BUFFER_LOCK[RTEXT_charPosition] = TEMPLATE_wrCell {
                c_char: RTEXT_char,
                c_colors: [Color::White, Color::Black],
            };
            RTEXT_charPosition[0] += 1
        }
        if RTEXT.t_lifetime == 255 {
            continue;
        }
        RTEXT.TEXT_tickdown();
    }

    DATA_LOCK
        .DATA_textItems
        .retain(|RTEXT| RTEXT.t_lifetime > 0)
}

/// # Render the world
fn r_util_world() {
    let mut BUFFER_LOCK = self::RENDER_mainBuffer.lock().unwrap();
    let DATA_LOCK = SYS_data.lock().unwrap();
    // First get vec of chunk references to not overload the system
    let r_workingChunkArray = DATA_LOCK.DATA_world.w_returnChunkArray(
        DATA_LOCK.DATA_player.p_chunk,
         system::SYS_REND_CHUNKRAD,
    );

    // Calc border offset
    // Player offset in chunk + Chunk radius offset - radius
    let r_workingBorderOffset = [
        // X
        (DATA_LOCK.DATA_player.p_pos[0] % system::SYS_CHUNK_X
            + system::SYS_REND_CHUNKRAD * system::SYS_CHUNK_X)
            - system::SYS_REND_WORLD_X,
        // Y
        (DATA_LOCK.DATA_player.p_pos[1] % system::SYS_CHUNK_Y
            + system::SYS_REND_CHUNKRAD * system::SYS_CHUNK_Y)
            - system::SYS_REND_WORLD_Y,
    ];

    // Quickset X position
    let mut w_bufferX: usize = 2;

    for XPOS in 0..system::SYS_REND_WORLDSIZE_X{

        // Quickset Y position
        let mut w_bufferY:usize = 2;

        // Just to not recalc every Y iter
        let idkfa_posX = r_workingBorderOffset[0] + XPOS;

        for YPOS in 0..system::SYS_REND_WORLDSIZE_Y{

            let idkfa_posY = r_workingBorderOffset[1] + YPOS;

            let w_cell = r_workingChunkArray[
                idkfa_posX/system::SYS_CHUNK_X + 
                idkfa_posY/system::SYS_CHUNK_Y * system::SYS_REND_CHUNKRADSIZE]
                    [[idkfa_posX%system::SYS_CHUNK_X, idkfa_posY%system::SYS_CHUNK_Y]];

            // Finally set the buffer cell
            // Gotta find a cleaner way for this
            BUFFER_LOCK[[w_bufferX, w_bufferY]] = TEMPLATE_wrCell{c_char:w_cell.c_char, c_colors:w_cell.c_color};
            w_bufferY += 1
        }
        w_bufferX += 1
    }
}

pub fn SYS_HANDLER_renderDebugStrs() {
    let mut DEBUG_LOCK = SYS_debug.lock().unwrap();
    let mut STDOUT_LOCK = stdout().lock();
    for DEBUGSTR in DEBUG_LOCK.DATA_debugItems.values_mut() {
        // Ignore these
        if &DEBUGSTR.t_string == "#MARK_FOR_DELETION" {
            continue;
        }

        write!(
            STDOUT_LOCK,
            "{} {}\r\n",
            &DEBUGSTR.t_string.clone().with(system::SYS_DEBUGCOLORS.0),
            &DEBUGSTR.t_values.clone().with(system::SYS_DEBUGCOLORS.1)
        )
        .unwrap();

        DEBUGSTR.TEXT_tickdown()
    }
    STDOUT_LOCK.flush().unwrap();
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
    pub c_colors: system::cellColors,
}
impl TEMPLATE_wrCell {
    pub fn new() -> Self {
        Self {
            c_char: ' ',
            c_colors: system::SYS_DEFCOLORS.into(),
        }
    }
}

/// # Position in Render Buffer
/// A selection of common positions for useage
///
/// You can also use your custom position with `POS_custom`
pub enum RENDER_position {
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
    POS_custom(system::coords),
}
impl RENDER_position {
    pub fn value(&self) -> system::coords {
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
            Self::POS_custom(POS) => POS,
        }
    }
}

/// # Render Buffer
///
/// Holds full size cells
struct RENDER_buffer {
    pub BUFFER_grid: Vec<TEMPLATE_wrCell>,
}
impl RENDER_buffer {
    pub fn new(IN_size: system::coords) -> Self {
        Self {
            BUFFER_grid: vec![TEMPLATE_wrCell::new(); IN_size[0] * IN_size[1]],
        }
    }
    pub fn reset(&mut self) {
        self.BUFFER_grid.fill(TEMPLATE_wrCell::new())
    }
}
impl Index<system::coords> for RENDER_buffer {
    type Output = TEMPLATE_wrCell;
    fn index(&self, index: system::coords) -> &Self::Output {
        &self.BUFFER_grid[index[0] + index[1] * system::SYS_REND_BUFFER_X]
    }
}
impl IndexMut<system::coords> for RENDER_buffer {
    fn index_mut(&mut self, index: system::coords) -> &mut Self::Output {
        &mut self.BUFFER_grid[index[0] + index[1] * system::SYS_REND_BUFFER_X]
    }
}