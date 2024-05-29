#![allow(nonstandard_style)]

use std::{ops::Range, time::Duration};

use crossterm::style::Color;

// # FULL FILE DISCLAIMER
// THIS WILL BE MOVED INTO A CUSTOMIZABLE `.json` FILE LATER ON
//
// SO THAT GITHUB WON'T SCREAM ABOUT INCOMPATIBILITY ERRORS
//
// AND SO THAT TESTING CAN BE DONE WITHOUT RECOMPILING

// How fast should game process everything
// DO NOT TOUCH SYS_TICKTIME!!!!
pub const SYS_TICKRATE: u8 = 8;
pub const SYS_TICKTIME: Duration = Duration::from_millis(1000 / SYS_TICKRATE as u64);

// Custom types so I don't peck it up
pub type vector2 = (usize, usize);
pub type colorSet = (crossterm::style::Color, crossterm::style::Color);


pub mod WORLD{
    use super::*;

    pub mod GENERAL{
        // World size in chunks
        pub const WORLD_X: usize = 8;
        pub const WORLD_Y: usize = 8;

        // Chunk size
        pub const WORLD_CHUNK_X: usize = 8;
        pub const WORLD_CHUNK_Y: usize = 8;

        // DO NOT TOUCH!!!
        // Full dimensions of the world
        pub const WORLD_GRID_X: usize = WORLD_X * WORLD_CHUNK_X;
        pub const WORLD_GRID_Y: usize = WORLD_Y * WORLD_CHUNK_Y;

    }

    pub mod GENERATION{
        use super::*;
        // Amount of ponds/lakes to generate Min-Max
        pub const GEN_POND_Q: Range<usize> = 4..6;

        // Size of pond/lake iterations Min-Max
        pub const GEN_POND_SIZE: Range<usize> = 3..10;

        // How deep should pond/lake iterations go Min-Max
        pub const GEN_POND_ITERS: Range<usize> = 6..8;

        // Amount of forests to generate Min-Max
        pub const GEN_FOREST_Q: Range<usize> = 4..8;

        // Size of forest iterations Min-Max
        pub const GEN_FOREST_SIZE: Range<usize> = 6..10;

        // How deep should forest iterations go Min-Max
        pub const GEN_FOREST_ITERS: Range<usize> = 5..8;
    }
}

pub mod RENDERER{

    // Render Buffer size
    // WARNING: Too high values may result in terminal scroll stutter
    pub const RENDER_BUFFER_X: usize = 48;
    pub const RENDER_BUFFER_Y: usize = 32;

    // Radius dimensions of the world screen
    pub const RENDER_WORLD_X: usize = 20;
    pub const RENDER_WORLD_Y: usize = 20;

    // DO NOT TOUCH!!
    // Full size of RENDERer world
    pub const RENDER_WORLDSIZE_X: usize = RENDER_WORLD_X * 2 + 1;
    pub const RENDER_WORLDSIZE_Y: usize = RENDER_WORLD_Y * 2 + 1;

    // Sets radius for chunks that should be loaded into renderer at once
    // The chunk player's in is always loaded
    // If you'll set it to 0 and report it as a bug I will punch you.
    pub const RENDER_CHUNKRAD: usize = 4;

    // DO NOT TOUCH!!
    // Full size of RENDERer chunks
    pub const RENDER_CHUNKRADSIZE: usize = RENDER_CHUNKRAD * 2 + 1;
}

pub mod MISC{
    use super::*;

    pub mod COLORS{
        use super::*;
        /// Default Render colors
        pub const COLORS_DEF: (Color, Color) = (Color::White, Color::Reset);

        /// Default debug colors
        pub const COLORS_DEBUG: (Color, Color) = (Color::White, Color::Yellow);
    }
    pub mod PATHS{
        use super::*;

        /// Default path to Debugs
        pub const PATH_DEBUG: &str = "./src/json/debug.json";

        /// Default path to Errors
        pub const PATH_ERROR: &str = "./src/json/error.json";
    }
}

struct SYS_COLOR{
    r: u8,
    g: u8,
    b: u8
}

/// # Common colors
/// Use `.raw` function to use with formatter
pub enum SYS_COMCOLORS {
    black,
    white,
    cyan,
    darkCyan,
    green,
    darkGreen,
    yellow,
    darkYellow,
    orange,
    darkOrange
}