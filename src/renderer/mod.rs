//! # Text renderer
//! ## Disclaimer
//! It will be replaced should I move onto Pixel renderer
//!
//! Although hopefully the new one will be <<compatible

use super::*;

mod world;
mod text;
mod render_util;


static RENDER_mainBuffer: Lazy<Mutex<RENDER_buffer>> = Lazy::new(|| {
    Mutex::new(RENDER_buffer::new((
        vars::RENDERER::RENDER_BUFFER_X,
        vars::RENDERER::RENDER_BUFFER_Y,
    )))
});

pub fn init() {
    let mut DEBUG_LOCK = statics::SYS_debug.lock().unwrap();

    // Bloody hell this is long
    'INIT_debugStr: {
        DEBUG_LOCK.DEBUG_items.insert(
            "#RENDER_frameTime".to_string(),
            debug::DEBUG_item::new(
                ".DEBUG_render/#RENDER_frameTime",
                MISC::PATHS::PATH_DEBUG,
                "",
                255
            ),
        );

        DEBUG_LOCK.DEBUG_items.insert(
            "#RENDER_worldTime".to_string(),
            debug::DEBUG_item::new(
                ".DEBUG_render/#RENDER_worldTime",
                MISC::PATHS::PATH_DEBUG,
                "",
                255
            ),
        );

        DEBUG_LOCK.DEBUG_items.insert(
            "#RENDER_convTime".to_string(),
            debug::DEBUG_item::new(
                ".DEBUG_render/#RENDER_convTime",
                MISC::PATHS::PATH_DEBUG,
                "",
                255
            ),
        );

        DEBUG_LOCK.DEBUG_items.insert(
            "#RENDER_borderTime".to_string(),
            debug::DEBUG_item::new(
                ".DEBUG_render/#RENDER_borderTime",
                MISC::PATHS::PATH_DEBUG,
                "",
                255
            ),
        );

        DEBUG_LOCK.DEBUG_items.insert(
            "#RENDER_textTime".to_string(),
            debug::DEBUG_item::new(
                ".DEBUG_render/#RENDER_textTime",
                MISC::PATHS::PATH_DEBUG,
                "",
                255
            ),
        );

        DEBUG_LOCK.DEBUG_items.insert(
            "#SSINIT_render".to_string(),
            debug::DEBUG_item::new(
                ".DEBUG_sys/.SYS_ssInit/#SSINIT_render",
                MISC::PATHS::PATH_DEBUG,
                "",
                40
            ),
        );
    }
}

/// # Render game
pub fn main() {
    let RENDER_start = time::Instant::now();
    let mut DEBUG_LOCK = statics::SYS_debug.lock().unwrap();

    'RENDER_renderWorld: {
        let loopStart = time::Instant::now();

        world::r_util_world();

        DEBUG_LOCK
        .DEBUG_items
        .get_mut("#RENDER_worldTime")
        .unwrap()
        .values = format!("{:?}", loopStart.elapsed())
    }

    // Set cell for the player
    // TODO: Clean up
    'RENDER_playerSet: {
        let DATA_LOCK = statics::SYS_data.lock().unwrap();

        RENDER_mainBuffer.lock().unwrap()[(
            (vars::RENDERER::RENDER_WORLD_X + 2),
            (vars::RENDERER::RENDER_WORLD_Y + 2),
        )] = TEMPLATE_wrCell {
            c_char: 'P',
            c_colors: DATA_LOCK.DATA_player.p_color,
        };
    }

    'RENDER_renderWorldBorder: {
        let loopStart = time::Instant::now();

        render_util::border::main(
            (1, 1),
            (vars::RENDERER::RENDER_WORLDSIZE_X, vars::RENDERER::RENDER_WORLDSIZE_Y),
        );

        DEBUG_LOCK
            .DEBUG_items
            .get_mut("#RENDER_borderTime")
            .unwrap()
            .values = format!("{:?}", loopStart.elapsed())
    }

    'RENDER_renderText: {
        let loopStart = time::Instant::now();

        text::render_textBox();

        DEBUG_LOCK
            .DEBUG_items
            .get_mut("#RENDER_textTime")
            .unwrap()
            .values = format!("{:?}", loopStart.elapsed())
    }
    
    // Print frame
    'RENDER_printFrame: {
        let loopStart = time::Instant::now();
        let mut STDOUT_LOCK = stdout().lock();
        let mut BUFFER_LOCK = RENDER_mainBuffer.lock().unwrap();
        
        let _ = execute!(
            STDOUT_LOCK,
            // Start sync
            terminal::BeginSynchronizedUpdate,
            // Clear the screen and get ready to print next frame
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        );
        
        // Print buffer to output
        for YPOS in 0..vars::RENDERER::RENDER_BUFFER_Y {
            for XPOS in 0..vars::RENDERER::RENDER_BUFFER_X {
                let RENDER_cell = BUFFER_LOCK[(XPOS, YPOS)];
                let _ = write!(
                    STDOUT_LOCK,
                    "{}",
                    RENDER_cell
                );
            }
            write!(STDOUT_LOCK, "\r\n").unwrap()
        }

        // End sync and push the frame
        let _ = STDOUT_LOCK.execute(terminal::EndSynchronizedUpdate);

        // Reset the buffer
        BUFFER_LOCK.reset();

        // And log the time
        DEBUG_LOCK
            .DEBUG_items
            .get_mut("#RENDER_convTime")
            .unwrap()
            .values = format!("{:?}", loopStart.elapsed());
    }

    // Log how long the entire process took
    DEBUG_LOCK
        .DEBUG_items
        .get_mut("#RENDER_frameTime")
        .unwrap()
        .values = format!("{:?}", RENDER_start.elapsed());

    // Drop the Debug lock, we're done here
    drop(DEBUG_LOCK);

    // Render debug stuff last, everything should be processed by this point
    text::render_debug();
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
    pub c_colors: types::colorSet,
}
impl TEMPLATE_wrCell {
    pub fn new() -> Self {
        Self {
            c_char: ' ',
            c_colors: vars::MISC::COLORS::COLORS_DEF,
        }
    }
}
impl fmt::Display for TEMPLATE_wrCell{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c_char.with(self.c_colors.0).on(self.c_colors.1))
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
    POS_custom(types::vector2),
}
impl RENDER_position {
    pub fn value(&self) -> types::vector2 {
        match *self {
            Self::None => (0, 0),
            Self::POS_middle => (vars::RENDERER::RENDER_BUFFER_X / 2, vars::RENDERER::RENDER_BUFFER_Y / 2),
            Self::POS_left => (0, vars::RENDERER::RENDER_BUFFER_Y / 2),
            Self::POS_right => (vars::RENDERER::RENDER_BUFFER_X - 1, vars::RENDERER::RENDER_BUFFER_Y / 2),
            Self::POS_top => (vars::RENDERER::RENDER_BUFFER_X / 2, 0),
            Self::POS_bottom => (vars::RENDERER::RENDER_BUFFER_X / 2, vars::RENDERER::RENDER_BUFFER_Y - 1),
            Self::POS_TL => (0, 0),
            Self::POS_TR => (vars::RENDERER::RENDER_BUFFER_X - 1, 0),
            Self::POS_BL => (0, vars::RENDERER::RENDER_BUFFER_Y - 1),
            Self::POS_BR => (vars::RENDERER::RENDER_BUFFER_X - 1, vars::RENDERER::RENDER_BUFFER_Y - 1),
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
    pub fn new(IN_size: types::vector2) -> Self {
        Self {
            BUFFER_dummyCell: TEMPLATE_wrCell::new(),
            BUFFER_grid: vec![TEMPLATE_wrCell::new(); IN_size.0 * IN_size.1],
        }
    }
    pub fn reset(&mut self) {
        self.BUFFER_grid.fill(TEMPLATE_wrCell::new())
    }
}
impl Index<types::vector2> for RENDER_buffer {
    type Output = TEMPLATE_wrCell;
    fn index(&self, index: types::vector2) -> &Self::Output {
        // Prevents writing out of bounds of the renderer
        if index.0 >= vars::RENDERER::RENDER_BUFFER_X || index.1 >= vars::RENDERER::RENDER_BUFFER_Y{
            return &self.BUFFER_dummyCell
        }
        &self.BUFFER_grid[index.0 + index.1 * vars::RENDERER::RENDER_BUFFER_X]
    }
}
impl IndexMut<types::vector2> for RENDER_buffer {
    fn index_mut(&mut self, index: types::vector2) -> &mut Self::Output {
        // Prevents writing out of bounds of the renderer
        if index.0 >= vars::RENDERER::RENDER_BUFFER_X || index.1 >= vars::RENDERER::RENDER_BUFFER_Y{
            return &mut self.BUFFER_dummyCell
        }
        &mut self.BUFFER_grid[index.0 + index.1 * vars::RENDERER::RENDER_BUFFER_X]
    }
}