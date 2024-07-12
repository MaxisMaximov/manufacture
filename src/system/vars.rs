#![allow(nonstandard_style)]

use super::*;

// # FULL FILE DISCLAIMER
// THIS WILL BE MOVED INTO A CUSTOMIZABLE `.json` FILE LATER ON
//
// SO THAT GITHUB WON'T SCREAM ABOUT INCOMPATIBILITY ERRORS
//
// AND SO THAT TESTING CAN BE DONE WITHOUT RECOMPILING

pub mod SYS{
    use super::*;

    // How fast should game process everything
    // DO NOT TOUCH SYS_TICKTIME!!!!
    pub const TICKRATE: u8 = 8;
    pub const TICKTIME: time::Duration = time::Duration::from_millis(1000 / TICKRATE as u64);
}


pub mod WORLD {

    pub mod GENERAL {
        // World size in chunks
        pub const WORLD_X: usize = 8;
        pub const WORLD_Y: usize = 8;

        // Chunk size
        pub const CHUNK_X: usize = 8;
        pub const CHUNK_Y: usize = 8;

        // DO NOT TOUCH!!!
        // Full dimensions of the world
        pub const GRID_X: usize = WORLD_X * CHUNK_X;
        pub const GRID_Y: usize = WORLD_Y * CHUNK_Y;
    }

    pub mod GENERATION {

        // Amount of ponds/lakes to generate Min-Max
        pub static GEN_POND_Q: (usize, usize) = (4, 6);

        // Size of pond/lake iterations Min-Max
        pub static GEN_POND_SIZE: (usize, usize) = (3, 10);

        // How deep should pond/lake iterations go Min-Max
        pub static GEN_POND_ITERS: (usize, usize) = (6, 8);

        // Amount of forests to generate Min-Max
        pub static GEN_FOREST_Q: (usize, usize) = (4, 8);

        // Size of forest iterations Min-Max
        pub static GEN_FOREST_SIZE: (usize, usize) = (6, 10);

        // How deep should forest iterations go Min-Max
        pub static GEN_FOREST_ITERS: (usize, usize) = (5, 8);
    }
}

pub mod RENDERER {

    // Render Buffer size
    // WARNING: Too high values may result in terminal scroll stutter
    pub const RENDER_BUFFER_X: usize = 48;
    pub const RENDER_BUFFER_Y: usize = 32;

    // Radius dimensions of the world screen
    pub const RENDER_WORLD_X: usize = 10;
    pub const RENDER_WORLD_Y: usize = 10;

    // DO NOT TOUCH!!
    // Full size of renderer world
    pub const RENDER_WORLDSIZE_X: usize = RENDER_WORLD_X * 2 + 1;
    pub const RENDER_WORLDSIZE_Y: usize = RENDER_WORLD_Y * 2 + 1;

    // Sets radius for chunks that should be loaded into renderer at once
    // The chunk player's in is always loaded
    // If you'll set it to 0 and report it as a bug I will punch you.
    pub const RENDER_CHUNKRAD: usize = 4;

    // DO NOT TOUCH!!
    // Full size of Renderer chunks
    pub const RENDER_CHUNKRADSIZE: usize = RENDER_CHUNKRAD * 2 + 1;
}

pub mod MISC {
    use super::*;

    pub mod COLORS {
        use super::*;
        /// Default Render colors
        /// (Foreground, Background)
        pub const COLORS_DEF: types::colorSet = (Color::White, Color::Black);

        /// Default debug colors
        /// (Main text, Values)
        pub const COLORS_DEBUG: types::colorSet = (Color::White, Color::Yellow);

        /// Default error colors
        /// (ErrorSpec, Rest)
        pub const COLORS_ERROR: types::colorSet = (Color::Red, Color::White);
    }
    pub mod PATHS {

        /// Default path to Debugs
        pub const PATH_DEBUG: &str = "./src/json/debug.json";
        
    }
}
pub mod PLAYER{
    // Step size while moving
    pub const PLAYER_STEP_SIZE: usize = 1;

    // Base health
    pub const PLAYER_BASE_HP: u16 = 100;

    // How far the player can 'leap'
    pub const PLAYER_LEAP_SIZE: usize = 4;
}