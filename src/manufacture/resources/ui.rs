use super::*;

pub struct UI_element{
    pub position: Vector2,
    pub content: String,
    pub request: Option<&'static str>,
    pub fg: Color,
    pub bg: Color
}

pub struct NodeTree<T>{
    root: Node<T>,
    maxDepth: u16 // The tree should NOT consist of 32k layers, if it does, you're doing something horribly wrong
}
pub struct Node<T>{
    val: T,
    depth: u16,
    _maxDepth: u16, // Yeah, each node carries the tree's max depth to not do super long recall chain
    nodes: Vec<Node<T>>
}
impl<T> Deref for Node<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}
impl<T> DerefMut for Node<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}