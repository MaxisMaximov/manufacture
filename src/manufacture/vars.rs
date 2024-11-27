// World size in chunks
pub const WORLD_X_MIN: isize = -5;
pub const WORLD_X_MAX: isize = 5;
pub const WORLD_Y_MIN: isize = -5;
pub const WORLD_Y_MAX: isize = 5;

pub const CHUNK_X: isize = 8;
pub const CHUNK_Y: isize = 8;

pub const RENDER_VIEWPORT_X_MIN: isize = -5;
pub const RENDER_VIEWPORT_X_MAX: isize = 5;
pub const RENDER_VIEWPORT_Y_MIN: isize = -5;
pub const RENDER_VIEWPORT_Y_MAX: isize = 5;

// Don't touch, full size of the Viewport render
pub const RENDER_VIEWPORT_X: usize = (RENDER_VIEWPORT_X_MAX - RENDER_VIEWPORT_X_MIN) as usize;
pub const RENDER_VIEWPORT_Y: usize = (RENDER_VIEWPORT_Y_MAX - RENDER_VIEWPORT_Y_MIN) as usize;

// Yeah smaller than the old buffer, realized I don't use most of it anyway lol
// Yet
pub const RENDER_BUFFER_X: usize = 20;
pub const RENDER_BUFFER_Y: usize = 20;

// To still render objects if they're "technically" in view
pub const RENDER_MARGIN: isize = 4;

