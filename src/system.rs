use std::{time::Duration, ops::Range};

// # FULL FILE DISCLAIMER
// THIS WILL BE MOVED INTO A CUSTOMIZABLE `.json` FILE LATER ON
// 
// SO THAT GITHUB WON'T SCREAM ABOUT INCOMPATIBILITY ERRORS
// 
// AND SO THAT TESTING CAN BE DONE WITHOUT RECOMPILING

pub const SYS_TICKRATE: u8 = 8;
pub const SYS_TICKTIME: Duration = Duration::from_millis(1000 / SYS_TICKRATE as u64);

// region: World data

    // region: General

        // World size in chunks
        pub const SYS_WORLD_X: usize = 8;
        pub const SYS_WORLD_Y: usize = 8;

        // Chunk size
        pub const SYS_CHUNK_X: usize = 8;
        pub const SYS_CHUNK_Y: usize = 8;

        // DO NOT TOUCH!!!
        // Full dimensions of the world
        pub const SYS_GRID_X: usize = SYS_WORLD_X * SYS_CHUNK_X;
        pub const SYS_GRID_Y: usize = SYS_WORLD_Y * SYS_CHUNK_Y;

    // endregion: General

    // region: Generation

        // region: Lakes

            // Amount of ponds/lakes to generate Min-Max
            pub const WORLD_POND_Q: Range<usize> = 4..6;

            // Size of pond/lake iterations Min-Max
            pub const WORLD_POND_SIZE: Range<usize> = 3..10;

            // How deep should pond/lake iterations go Min-Max
            pub const WORLD_POND_ITERS: Range<usize> = 6..8;

            pub const WORLD_POND_DEEP_THRES: usize = 3;

        // endregion: Lakes

        // region: Forests

            // Amount of forests to generate Min-Max
            pub const WORLD_FOREST_Q: Range<usize> = 4..8;

            // Size of forest iterations Min-Max
            pub const WORLD_FOREST_SIZE: Range<usize> = 6..10;

            // How deep should forest iterations go Min-Max
            pub const WORLD_FOREST_ITERS: Range<usize> = 5..8;

        // endregion: Forests

    // endregion: Generation

// endregion: World data



// region: Renderer data

    // Render Buffer size
    // WARNING: Too high values may result in terminal scroll stutter
    pub const SYS_REND_BUFFER_X: usize = 48;
    pub const SYS_REND_BUFFER_Y: usize = 32;

    // Dimensions of the world screen
    pub const SYS_REND_WORLD_X: usize = 24;
    pub const SYS_REND_WORLD_Y: usize = 24;

    // Sets how many chunks should be loaded into Renderer at once
    // KEEP IT TO ODD NUMBERS
    // It will crash the game if it's not
    // Also if you'll set it to 0 and report it as a bug I will punch you.
    pub const SYS_REND_CHUNK_X: usize = 3;
    pub const SYS_REND_CHUNK_Y: usize = 3;

// endregion: Renderer data