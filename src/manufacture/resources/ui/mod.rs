use super::*;

mod specials;
pub use specials::Special;

pub struct UINode{
    pub tag: Tag,
    pub calculated: Calculated
}

pub struct Calculated{
    pub position: (usize, usize),
    pub box_bounding: (usize, usize)
}

pub enum Tag{
    None,
    Text(String),
    Special(Box<dyn specials::Special>)
}