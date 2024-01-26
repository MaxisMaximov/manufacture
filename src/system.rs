use std::time::Duration;

pub const SYS_TICKRATE: u8 = 8;
pub const SYS_TICKTIME: Duration = Duration::from_millis(1000 / SYS_TICKRATE as u64);

// World size
// DO NOT RELY ON THIS
// It'll change with Chunk system
pub const SYS_WORLD_X: usize = 8;
pub const SYS_WORLD_Y: usize = 8;

// Chunk size
pub const SYS_CHUNK_X: usize = 8;
pub const SYS_CHUNK_Y: usize = 8;

// DO NOT TOUCH!!!
// Full dimensions of the world
pub const SYS_GRID_X: usize = SYS_WORLD_X * SYS_CHUNK_X;
pub const SYS_GRID_Y: usize = SYS_WORLD_Y * SYS_CHUNK_Y;

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