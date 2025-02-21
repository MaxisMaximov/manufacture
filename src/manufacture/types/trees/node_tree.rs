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
    pub fn new(IN_val: T, IN_depth: u16) -> Self{
        Self{
            val: IN_val,
            depth: IN_depth,
            nodes: Vec::new()
        }
    }

    pub fn newRoot(IN_val: T) -> Self{
        Self::new(IN_val, 0)
    }

    pub fn addNode(&mut self, IN_val: T) -> usize{
        self.nodes.push(Self::new(IN_val, self.depth + 1));
        self.nodes.len() - 1
    }
    pub fn addNodeRaw(&mut self, IN_node: Node<T>){
        self.nodes.push(IN_node);
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

    pub fn withNodes<F: Fn(&mut Node<T>)>(mut self, IN_function: F) -> Self{
        IN_function(&mut self);
        self
    }
    pub fn map<F: Fn(&mut Node<T>)>(&mut self, IN_function: F) -> &mut Self{
        IN_function(self);
        self
    }

    pub fn clearNodes(&mut self){
        self.nodes.clear();
    }
}