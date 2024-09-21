use super::*;

// Custom types so I don't peck it up

/// (X, Y)
pub type vector2 = (usize, usize);
/// (X, Y, Z)
pub type vector3 = (usize, usize, usize);
/// (Foreground, Background)
pub type colorSet = (Color, Color);

pub struct styleSet{
    pub ch: char,
    pub fg: Color,
    pub bg: Color
}

/// Color struct
/// (R, G, B)
pub struct SYS_COLOR(u8, u8, u8);