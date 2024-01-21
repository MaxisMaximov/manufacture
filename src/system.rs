use std::time::Duration;

pub const SYS_TICKRATE: u8 = 8;
pub const SYS_TICKTIME: Duration = Duration::from_millis(1000 / SYS_TICKRATE as u64);

// World size
// DO NOT RELY ON THIS
// It'll change with Chunk system
pub const SYS_GRID_X: usize = 16;
pub const SYS_GRID_Y: usize = 16;

// Render Buffer size
// WARNING: Too high values may result in terminal scroll stutter
pub const SYS_REND_X: usize = 48;
pub const SYS_REND_Y: usize = 32;