//! # Text renderer
//! ## Disclaimer
//! It will be replaced should I move onto Pixel renderer
//!
//! Although hopefully the new one will be <<compatible

use std::io::BufWriter;

use super::*;

mod world;
mod text;
mod render_util;

pub mod widgets;

static buffer: Lazy<Mutex<buffer_master>> = Lazy::new(|| {
    Mutex::new(buffer_master::new((
        vars::RENDERER::RENDER_BUFFER_X,
        vars::RENDERER::RENDER_BUFFER_Y,
    )))
});

pub fn init() {
    let mut DEBUG_LOCK = statics::debug.lock().unwrap();

    // Bloody hell this is long
    'INIT_debugStr: {
        DEBUG_LOCK.inner.insert(
            ">RENDER_frameTime".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".RENDER/#frameTime",
                vars::MISC::PATHS::PATH_DEBUG,
                &[("{time}", "".to_owned())],
                255
            ),
        );

        DEBUG_LOCK.inner.insert(
            ">RENDER_worldTime".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".RENDER/#worldTime",
                vars::MISC::PATHS::PATH_DEBUG,
                &[("{time}", "".to_owned())],
                255
            ),
        );

        DEBUG_LOCK.inner.insert(
            ">RENDER_convTime".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".RENDER/#convTime",
                vars::MISC::PATHS::PATH_DEBUG,
                &[("{time}", "".to_owned())],
                255
            ),
        );

        DEBUG_LOCK.inner.insert(
            ">RENDER_borderTime".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".RENDER/#borderTime",
                vars::MISC::PATHS::PATH_DEBUG,
                &[("{time}", "".to_owned())],
                255
            ),
        );

        DEBUG_LOCK.inner.insert(
            ">RENDER_textTime".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".RENDER/#textTime",
                vars::MISC::PATHS::PATH_DEBUG,
                &[("{time}", "".to_owned())],
                255
            ),
        );

        DEBUG_LOCK.inner.insert(
            ">SYS_SSINIT_render".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".SYS/.SYS_ssInit/#SSINIT_render",
                vars::MISC::PATHS::PATH_DEBUG,
                &[],
                40
            ),
        );
    }
}

/// # Render game
pub fn main() {
    let TIMER_renderStart = time::Instant::now();
    let mut DEBUG_LOCK = statics::debug.lock().unwrap();

    'RENDER_renderWorld: {
        let loopStart = time::Instant::now();

        world::r_util_world();

        DEBUG_LOCK
        .inner
        .get_mut(">RENDER_worldTime")
        .unwrap()
        .values[0].1 = format!("{:?}", loopStart.elapsed())
    }

    // Set cell for the player
    // TODO: Clean up
    'RENDER_playerSet: {
        let DATA_LOCK = statics::data.lock().unwrap();

        buffer.lock().unwrap()[(
            (vars::RENDERER::RENDER_WORLD_X + 2),
            (vars::RENDERER::RENDER_WORLD_Y + 2),
        )] = render_cell {
            char: 'P',
            colors: DATA_LOCK.player.color,
        };
    }

    'RENDER_renderWorldBorder: {
        let loopStart = time::Instant::now();

        render_util::border::main(
            (1, 1),
            (vars::RENDERER::RENDER_WORLDSIZE_X, vars::RENDERER::RENDER_WORLDSIZE_Y),
        );

        DEBUG_LOCK
            .inner
            .get_mut(">RENDER_borderTime")
            .unwrap()
            .values[0].1 = format!("{:?}", loopStart.elapsed())
    }

    'RENDER_renderText: {
        let loopStart = time::Instant::now();

        text::textBoxes();

        DEBUG_LOCK
            .inner
            .get_mut(">RENDER_textTime")
            .unwrap()
            .values[0].1 = format!("{:?}", loopStart.elapsed())
    }
    
    // Print frame
    'RENDER_printFrame: {
        let loopStart = time::Instant::now();

        // Lock and load
        let mut STDOUT_LOCK = BufWriter::new(stdout().lock());
        let mut BUFFER_LOCK = buffer.lock().unwrap();
        
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
                let _ = write!(STDOUT_LOCK, "{}", BUFFER_LOCK[(XPOS, YPOS)]);
            }

            let _ = write!(STDOUT_LOCK, "\r\n");
        }

        // We're done, reset the buffer
        BUFFER_LOCK.reset();
        
        // Reset the colors
        let _ = write!(STDOUT_LOCK, "{}", "".with(vars::MISC::COLORS::COLORS_DEF.0).on(vars::MISC::COLORS::COLORS_DEF.1));
        
        // End sync and push the frame
        let _ = STDOUT_LOCK.execute(terminal::EndSynchronizedUpdate);

        // And log the time
        DEBUG_LOCK
            .inner
            .get_mut(">RENDER_convTime")
            .unwrap()
            .values[0].1 = format!("{:?}", loopStart.elapsed());
    }

    // Log how long the entire process took
    DEBUG_LOCK
        .inner
        .get_mut(">RENDER_frameTime")
        .unwrap()
        .values[0].1 = format!("{:?}", TIMER_renderStart.elapsed());

    // Drop the Debug lock, we're done here
    drop(DEBUG_LOCK);

    // Render debug stuff last, everything should be processed by this point
    text::debug();
}

/// # Render Buffer Cell
///
/// Values:
///
/// * Character
/// * Colors for character and background
#[derive(Clone, Copy)]
struct render_cell {
    pub char: char,
    pub colors: types::colorSet,
}
impl render_cell {
    pub fn new() -> Self {
        Self {
            char: ' ',
            colors: vars::MISC::COLORS::COLORS_DEF,
        }
    }
}
impl fmt::Display for render_cell{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char.with(self.colors.0).on(self.colors.1))
    }
}

/// # Render Buffer
///
/// Holds full size cells
struct buffer_master {
    BUFFER_dummyCell: render_cell,
    pub BUFFER_grid: Vec<render_cell>,
}
impl buffer_master {
    pub fn new(IN_size: types::vector2) -> Self {
        Self {
            BUFFER_dummyCell: render_cell::new(),
            BUFFER_grid: vec![render_cell::new(); IN_size.0 * IN_size.1],
        }
    }
    pub fn reset(&mut self) {
        self.BUFFER_grid.fill(render_cell::new())
    }
}
impl Index<types::vector2> for buffer_master {
    type Output = render_cell;
    fn index(&self, index: types::vector2) -> &Self::Output {
        // Prevents writing out of bounds of the renderer
        if index.0 >= vars::RENDERER::RENDER_BUFFER_X || index.1 >= vars::RENDERER::RENDER_BUFFER_Y{
            return &self.BUFFER_dummyCell
        }
        &self.BUFFER_grid[index.0 + index.1 * vars::RENDERER::RENDER_BUFFER_X]
    }
}
impl IndexMut<types::vector2> for buffer_master {
    fn index_mut(&mut self, index: types::vector2) -> &mut Self::Output {
        // Prevents writing out of bounds of the renderer
        if index.0 >= vars::RENDERER::RENDER_BUFFER_X || index.1 >= vars::RENDERER::RENDER_BUFFER_Y{
            return &mut self.BUFFER_dummyCell
        }
        &mut self.BUFFER_grid[index.0 + index.1 * vars::RENDERER::RENDER_BUFFER_X]
    }
}