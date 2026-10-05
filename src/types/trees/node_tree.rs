use std::ops::{Deref, DerefMut};

pub struct Node<T>{
    val: T,
    pub depth: u16,
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
    pub fn new(Val: T, Depth: u16) -> Self{
        Self{
            val: Val,
            depth: Depth,
            nodes: Vec::new()
        }
    }

    pub fn new_root(Val: T) -> Self{
        Self::new(Val, 0)
    }

    pub fn add_node(&mut self, Val: T) -> usize{
        self.nodes.push(Self::new(Val, self.depth + 1));
        self.nodes.len() - 1
    }
    pub fn add_node_raw(&mut self, Node: Node<T>){
        self.nodes.push(Node);
    }
    pub fn remove_node(&mut self, Index: usize) -> Node<T>{
        self.nodes.remove(Index)
    }

    pub fn node_count(&self, Recursive: bool) -> usize{
        if self.nodes.is_empty(){
            return 0
        }

        let mut Count = self.nodes.len();
        
        if Recursive{
            for node in self.nodes.iter(){
                Count += node.node_count(true)
            }
        }

        Count
    }

    pub fn get_node(&self, Index: usize) -> Option<&Node<T>>{
        self.nodes.get(Index)
    }
    pub fn get_node_mut(&mut self, Index: usize) -> Option<&mut Node<T>>{
        self.nodes.get_mut(Index)
    }

    pub fn with<F: Fn(&mut Node<T>)>(mut self, f: F) -> Self{
        f(&mut self);
        self
    }
    pub fn map<F: Fn(&mut Node<T>)>(&mut self, f: F) -> &mut Self{
        f(self);
        self
    }

    pub fn clear(&mut self){
        self.nodes.clear();
    }
}