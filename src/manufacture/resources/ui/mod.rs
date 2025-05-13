use super::*;
// Re-exports for convenience sake
pub use arena_tree::{ArenaTree, Handle, Node, Token};

pub struct UINode{
    pub tag: Tag,
    pub calculated: Calculated
}

pub struct Calculated{
    pub pos: Option<Vector2>,
    pub size: Option<(usize, usize)>
}

pub enum Tag{
    None,
    Text(String)
}