#![allow(nonstandard_style)]
#![allow(unused_labels)]
#![warn(unused_crate_dependencies)]

use std::thread::sleep;

pub use once_cell::sync::Lazy;
pub use crossterm::{style::{Color, Stylize}, *};
use vars::MISC;
pub use std::{collections::HashMap, sync::Mutex, fmt, io::{stdout, Write}, time, ops::{Index, IndexMut}};

// Define all subsystems
mod system;
mod logic;
mod renderer;
mod data;

// These are system stuff so yeah
pub use system::*;

// START HERE
fn main() {
    statics::debug.lock().unwrap().DEBUG_items.insert(
        ">SYS_SSINIT_data".to_string(),
        debug::debug_item::new(
            debug::class::info,
            ".SYS/.SSINIT/#data",
            MISC::PATHS::PATH_DEBUG,
            &[],
            40,
        ),
    );

    statics::debug.lock().unwrap().DEBUG_items.insert(
        ">SYS_processSpeed".to_string(),
        debug::debug_item::new(
            debug::class::info,
            ".SYS/#processSpeed",
            MISC::PATHS::PATH_DEBUG,
            &[("{time}", "".to_owned())],
            255,
        ),
    );

    // Switch to Raw Mode
    terminal::enable_raw_mode().unwrap();

    // Enter alternate screen and hide the cursor
    let _ = execute!(stdout(), 
        terminal::EnterAlternateScreen,
        terminal::SetSize(vars::RENDERER::RENDER_BUFFER_X as u16, vars::RENDERER::RENDER_BUFFER_Y as u16),
        terminal::SetTitle("manufacture"),
        cursor::Hide
    );

    // Generate new world
    // Commented out cuz for whatever reason it gets stuck in loop
    // Will fix it with new world gen
    //data.lock().unwrap().DATA_world.w_generateRandom();

    // "Initialize" the subsystems
    renderer::init();
    logic::init();
    input::init();
    json::init();

    // # THE GAME LOOP
    loop {
        // Start the timer
        let loopStart: time::Instant = time::Instant::now();

        // Get player input
        input::main();

        // Process the game
        logic::main();

        // Render everything
        renderer::main();

        // Log how long it took to process everything
        statics::debug.lock().unwrap().DEBUG_items
            .get_mut(">SYS_processSpeed")
            .unwrap()
            .values[0].1 = format!("{:?}", loopStart.elapsed());

        let loop_elapsedTime: time::Duration = loopStart.elapsed();
        if loop_elapsedTime < vars::SYS::TICKTIME {
            sleep(vars::SYS::TICKTIME - loop_elapsedTime)
        }
    }
}

/// # Cache type
/// Allows you to store a selected type of cache
///
/// Can be extended for any other cache type you want
pub enum CACHE_TYPE {
    usize(usize),
    u8(u8),
    vec2(types::vector2),
    vec3(types::vector3),
    interactCode(logic::GAME_interactions),
}

/// # Text/Debug struct
///
/// TextItem and DebugItem struct in one
///
/// `t_position` is ignored for DebugItems
pub struct IDDQD_textItem {
    pub t_position: renderer::RENDER_position,
    pub t_string: String,
    pub t_values: String,
    pub t_lifetime: u16,
    pub t_markForDel: bool,
}
impl IDDQD_textItem {
    /// # Create new TextItem
    ///
    /// The one used to place text somewhere in the game
    pub fn new(
        IN_pos: renderer::RENDER_position,
        IN_text: &str,
        IN_values: &str,
        IN_lifetime: u16,
    ) -> Self {
        // Check if it's a debug string
        let idkfa_string = if IN_text.starts_with('.') {
            json::debugStr(IN_text, vars::MISC::PATHS::PATH_DEBUG).unwrap_or(IN_text.to_string())
        } else {
            IN_text.to_string()
        };
        Self {
            t_position: IN_pos,
            t_string: idkfa_string,
            t_values: IN_values.to_string(),
            t_lifetime: IN_lifetime,
            t_markForDel: false,
        }
    }

    /// Tickdown lifetime  
    /// Just to make it clean
    pub fn TEXT_tickdown(&mut self) {
        // If it's marked for del just ignore
        if self.t_markForDel {
            return;
        }
        // If it's ""permament"" then don't do anything
        if self.t_lifetime == 255 {
            return;
        }
        // If lifetime is 0, mark for deletion
        if self.t_lifetime == 0 {
            self.t_markForDel = true;
            return;
        }
        self.t_lifetime -= 1;
    }
}
// Display for normal textboxes
impl fmt::Display for IDDQD_textItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            &self.t_string.clone().with(vars::MISC::COLORS::COLORS_DEF.0),
            &self.t_values.clone().with(vars::MISC::COLORS::COLORS_DEF.1)
        )
    }
}