use std::ops::{Index, IndexMut, Deref, DerefMut};

use crossterm::style::Color;

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

pub struct DoubleDArray<T, const X: usize, const Y: usize>{
    pub inner: [[T; X]; Y], // RUST PLEASE LET ME USE CONST EXPRESSIONS WITH GENERICS
    dummyT: T
}
impl<T: Default + Copy, const X: usize, const Y: usize> DoubleDArray<T, X, Y>{
    pub fn new() -> Self{
        Self{
            inner: [[T::default(); X]; Y],
            dummyT: T::default()
        }
    }
}
impl<T, const X: usize, const Y: usize> Index<Vector2> for DoubleDArray<T, X, Y>{
    type Output = T;

    fn index(&self, index: Vector2) -> &Self::Output {
        if let Some(ROW) = self.inner.get((index.1 as usize).wrapping_add(Y/2)){
            if let Some(TYPE) = ROW.get((index.0 as usize).wrapping_add(X/2)){
                return TYPE
            }
        }
        &self.dummyT
    }
}
impl<T, const X: usize, const Y: usize> IndexMut<Vector2> for DoubleDArray<T, X, Y>{
    fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
        if let Some(ROW) = self.inner.get_mut((index.1 as usize).wrapping_add(Y/2)){
            if let Some(TYPE) = ROW.get_mut((index.0 as usize).wrapping_add(X/2)){
                return TYPE
            }
        }
        &mut self.dummyT
    }
}

pub struct Node<T>{
    val: T,
    pub depth: u16,
    pub maxDepth: u16, // Yeah, each node carries the tree's max depth to not do super long recall chain
    pub nodes: Vec<Node<T>>
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
impl<T> Node<T>{
    pub fn new(IN_val: T, IN_depth: u16, IN_maxDepth: u16) -> Self{
        Self{
            val: IN_val,
            depth: IN_depth,
            maxDepth: IN_maxDepth,
            nodes: Vec::new()
        }
    }

    pub fn addNode(&mut self, IN_val: T) -> Result<usize, ()>{
        if self.depth >= self.maxDepth {
            return Err(())
        }
        self.nodes.push(Self::new(IN_val, self.depth + 1, self.maxDepth));
        Ok(self.nodes.len() - 1)
    }
    pub fn removeNode(&mut self, IN_id: usize) -> Node<T>{
        self.nodes.remove(IN_id)
    }

    pub fn nodeCount(&self, IN_recursive: bool) -> usize{
        if self.nodes.is_empty(){
            return 0
        }

        let mut OUT_count = self.nodes.len();
        
        if IN_recursive{
            for NODE in self.nodes.iter(){
                OUT_count += NODE.nodeCount(true)
            }
        }

        OUT_count
    }
    pub fn getNode(&self, IN_id: usize) -> Option<&Node<T>>{
        self.nodes.get(IN_id)
    }
    pub fn getNodeMut(&mut self, IN_id: usize) -> Option<&mut Node<T>>{
        self.nodes.get_mut(IN_id)
    }
}