use crossterm::style::Color;

pub mod cartesian;
pub mod trees;

pub use cartesian::*;
pub use trees::*;

pub type Vector2 = (isize, isize);

#[derive(Clone, Copy)]
pub struct StyleSet{
    pub ch: char,
    pub fg: Color,
    pub bg: Color
}
impl Default for StyleSet{
    fn default() -> Self {
        Self { ch: ' ', fg: Color::White, bg: Color::Black }
    }
}