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

// This is system stuff so yeah
pub use system::*;

// START HERE
fn main() {
    statics::debug.lock().unwrap().inner.insert(
        ">SYS_SSINIT_data".to_string(),
        debug::debug_item::new(
            debug::class::info,
            ".SYS/.SSINIT/#data",
            MISC::PATHS::PATH_DEBUG,
            &[],
            40,
        ),
    );

    statics::debug.lock().unwrap().inner.insert(
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
        statics::debug.lock().unwrap().inner
            .get_mut(">SYS_processSpeed")
            .unwrap()
            .values[0].1 = format!("{:?}", loopStart.elapsed());

        let loop_elapsedTime: time::Duration = loopStart.elapsed();
        if loop_elapsedTime < vars::SYS::TICKTIME {
            sleep(vars::SYS::TICKTIME - loop_elapsedTime)
        }
    }
}