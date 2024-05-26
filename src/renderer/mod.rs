use crossterm::{
    cursor::MoveTo,
    style::{Color, Stylize},
    terminal::{BeginSynchronizedUpdate, Clear, EndSynchronizedUpdate},
    ExecutableCommand,
};
use std::{
    io::{stdout, Write},
    ops::{Index, IndexMut},
    time::Instant,
};

use crate::*;

mod render_world;
mod render_text;
mod render_util;

/// # Text renderer
/// ## Disclaimer
/// It will be replaced should I move onto Pixel renderer
///
/// Although hopefully the new one will be <<compatible

static RENDER_mainBuffer: Lazy<Mutex<RENDER_buffer>> = Lazy::new(|| {
    Mutex::new(RENDER_buffer::new((
        RENDERER::RENDER_BUFFER_X,
        RENDERER::RENDER_BUFFER_Y,
    )))
});

pub fn init() {
    let mut DEBUG_LOCK = SYS_debug.lock().unwrap();
    'INIT_debugStr: {
        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_frameTime".to_string(),
            IDDQD_textItem::new(RENDER_position::None,".DEBUG_render/#RENDER_frameTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_worldTime".to_string(),
            IDDQD_textItem::new(RENDER_position::None,".DEBUG_render/#RENDER_worldTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_convTime".to_string(),
            IDDQD_textItem::new(RENDER_position::None,".DEBUG_render/#RENDER_convTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_borderTime".to_string(),
            IDDQD_textItem::new(RENDER_position::None,".DEBUG_render/#RENDER_borderTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#RENDER_textTime".to_string(),
            IDDQD_textItem::new(RENDER_position::None,".DEBUG_render/#RENDER_textTime", "", 255),
        );

        DEBUG_LOCK.DATA_debugItems.insert(
            "#SSINIT_render".to_string(),
            IDDQD_textItem::new(RENDER_position::None,".DEBUG_sys/.SYS_ssInit/#SSINIT_render", "", 40),
        );
    }
}

/// # Render game
pub fn main() {
    let RENDER_start = Instant::now();
    let mut DEBUG_LOCK = SYS_debug.lock().unwrap();

    'RENDER_renderWorld: {
        let loopStart = Instant::now();

        render_world::r_util_world();

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

        RENDER_mainBuffer.lock().unwrap()[(
            (RENDERER::RENDER_WORLD_X + 2),
            (RENDERER::RENDER_WORLD_Y + 2),
        )] = TEMPLATE_wrCell {
            c_char: 'P',
            c_colors: DATA_LOCK.DATA_player.p_color,
        };
    }

    'RENDER_renderText: {
        let loopStart = Instant::now();

        render_text::render_textBox();

        DEBUG_LOCK
        .DATA_debugItems
        .get_mut("#RENDER_textTime")
        .unwrap()
        .t_values = format!("{:?}", loopStart.elapsed())
    }

    'RENDER_renderWorldBorder: {
        let loopStart = Instant::now();

        render_util::util_border::main(
            (1, 1),
            (RENDERER::RENDER_WORLD_X * 2 + 1, RENDERER::RENDER_WORLD_Y * 2 + 1),
        );

        DEBUG_LOCK
        .DATA_debugItems
        .get_mut("#RENDER_borderTime")
        .unwrap()
        .t_values = format!("{:?}", loopStart.elapsed())
    }
    
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
        for YPOS in 0..RENDERER::RENDER_BUFFER_Y {
            for XPOS in 0..RENDERER::RENDER_BUFFER_X {
                let RENDER_cell = BUFFER_LOCK[(XPOS, YPOS)];
                let _ = write!(
                    STDOUT_LOCK,
                    "{}",
                    RENDER_cell
                        .c_char
                        .with(RENDER_cell.c_colors.0)
                        .on(RENDER_cell.c_colors.1)
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

    // Drop the Debug lock, we're done here
    drop(DEBUG_LOCK);

    // Render debug stuff last, everything should be processed by this point
    render_text::render_debug();
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
    pub c_colors: colorSet,
}
impl TEMPLATE_wrCell {
    pub fn new() -> Self {
        Self {
            c_char: ' ',
            c_colors: MISC::COLORS::COLORS_DEF,
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
    POS_custom(vector2),
}
impl RENDER_position {
    pub fn value(&self) -> vector2 {
        match *self {
            Self::None => (0, 0),
            Self::POS_middle => (RENDERER::RENDER_BUFFER_X / 2, RENDERER::RENDER_BUFFER_Y / 2),
            Self::POS_left => (0, RENDERER::RENDER_BUFFER_Y / 2),
            Self::POS_right => (RENDERER::RENDER_BUFFER_X - 1, RENDERER::RENDER_BUFFER_Y / 2),
            Self::POS_top => (RENDERER::RENDER_BUFFER_X / 2, 0),
            Self::POS_bottom => (RENDERER::RENDER_BUFFER_X / 2, RENDERER::RENDER_BUFFER_Y - 1),
            Self::POS_TL => (0, 0),
            Self::POS_TR => (RENDERER::RENDER_BUFFER_X - 1, 0),
            Self::POS_BL => (0, RENDERER::RENDER_BUFFER_Y - 1),
            Self::POS_BR => (RENDERER::RENDER_BUFFER_X - 1, RENDERER::RENDER_BUFFER_Y - 1),
            Self::POS_custom(POS) => POS,
        }
    }
}

/// # Render Buffer
///
/// Holds full size cells
struct RENDER_buffer {
    BUFFER_dummyCell: TEMPLATE_wrCell,
    pub BUFFER_grid: Vec<TEMPLATE_wrCell>,
}
impl RENDER_buffer {
    pub fn new(IN_size: vector2) -> Self {
        Self {
            BUFFER_dummyCell: TEMPLATE_wrCell::new(),
            BUFFER_grid: vec![TEMPLATE_wrCell::new(); IN_size.0 * IN_size.1],
        }
    }
    pub fn reset(&mut self) {
        self.BUFFER_grid.fill(TEMPLATE_wrCell::new())
    }
}
impl Index<vector2> for RENDER_buffer {
    type Output = TEMPLATE_wrCell;
    fn index(&self, index: vector2) -> &Self::Output {
        // Prevents writing out of bounds of the renderer
        if index.0 >= RENDERER::RENDER_BUFFER_X || index.1 >= RENDERER::RENDER_BUFFER_Y{
            return &self.BUFFER_dummyCell
        }
        &self.BUFFER_grid[index.0 + index.1 * RENDERER::RENDER_BUFFER_X]
    }
}
impl IndexMut<vector2> for RENDER_buffer {
    fn index_mut(&mut self, index: vector2) -> &mut Self::Output {
        // Prevents writing out of bounds of the renderer
        if index.0 >= RENDERER::RENDER_BUFFER_X || index.1 >= RENDERER::RENDER_BUFFER_Y{
            return &mut self.BUFFER_dummyCell
        }
        &mut self.BUFFER_grid[index.0 + index.1 * RENDERER::RENDER_BUFFER_X]
    }
}