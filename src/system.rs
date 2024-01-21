use std::time::Duration;

pub const SYS_TICKRATE: u8 = 8;
pub const SYS_TICKTIME: Duration = Duration::from_millis(1000 / SYS_TICKRATE as u64);
pub const SYS_GRID_X: usize = 16;
pub const SYS_GRID_Y: usize = 16;
pub const SYS_REND_X: usize = 48;
pub const SYS_REND_Y: usize = 32;