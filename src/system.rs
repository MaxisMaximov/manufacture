use std::{time::Duration, ops::RangeInclusive};

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

        // Amount of ponds/lakes to generate Min-Max
        pub const WORLD_POND_Q: RangeInclusive<usize> = 4..=12;

        // Size of ponds/lakes Min-Max
        pub const WORLD_POND_SIZE: RangeInclusive<usize> = 3..=5;

        // How deep should pond iterations go Min-Max
        pub const WORLD_PONG_ITERS: RangeInclusive<usize> = 3..=5;

        // How many "leafs" should each iteration have Min-Max
        pub const WORLD_POND_ITERS_LEAF: RangeInclusive<usize> = 2..=4;

    // endregion: Generation

// endregion: World data



// region: Renderer data

    // Render Buffer size
    // WARNING: Too high values may result in terminal scroll stutter
    pub const SYS_REND_BUFFER_X: usize = 48;
    pub const SYS_REND_BUFFER_Y: usize = 32;

    // Dimensions of the world screen
    pub const SYS_REND_WORLD_X: usize = 15;
    pub const SYS_REND_WORLD_Y: usize = 15;

    // DO NOT TOUCH!!!
    // Sets how many chunks should be loaded into Renderer at once
    // Gets minimal size for given world screen dimensions
    pub const SYS_REND_CHUNK_X: usize = SYS_REND_WORLD_X / SYS_CHUNK_X + 2;
    pub const SYS_REND_CHUNK_Y: usize = SYS_REND_WORLD_Y / SYS_CHUNK_Y + 2;

// endregion: Renderer data