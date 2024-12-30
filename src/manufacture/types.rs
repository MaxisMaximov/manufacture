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

pub type inv_Item = u8;

pub struct CartesianGrid<T>{ // It's Cartesian and it's a grid -- Just to not confuse with *proper* 2D arrays
    pub inner: Vec<T>,
    size: (usize, usize),
    dummyT: T
}
impl<T: Default + Copy> CartesianGrid<T>{
    pub fn new(IN_x: usize, IN_y: usize) -> Self{
        Self{
            inner: vec![T::default(); IN_x * IN_y],
            size: (IN_x, IN_y),
            dummyT: T::default()
        }
    }
}
impl<T> Index<Vector2> for CartesianGrid<T>{
    type Output = T;

    fn index(&self, index: Vector2) -> &Self::Output {

        let X = (index.0 as usize).wrapping_add(self.size.0/2);
        let Y = (index.1 as usize).wrapping_add(self.size.1/2);

        if X >= self.size.0 || Y >= self.size.1{
            return &self.dummyT
        }

        if let Some(CELL) = self.inner.get(X + Y * self.size.0){
            return CELL
        }
        &self.dummyT
    }
}
impl<T> IndexMut<Vector2> for CartesianGrid<T>{
    fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
        
        let X = (index.0 as usize).wrapping_add(self.size.0/2);
        let Y = (index.1 as usize).wrapping_add(self.size.1/2);

        if X >= self.size.0 || Y >= self.size.1{
            return &mut self.dummyT
        }

        if let Some(CELL) = self.inner.get_mut(X + Y * self.size.0){
            return CELL
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