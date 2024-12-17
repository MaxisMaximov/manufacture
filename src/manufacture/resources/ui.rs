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
#[derive(Clone)]
pub struct Node<T>{
    val: T,
    pub depth: u16,
    maxDepth: u16, // Yeah, each node carries the tree's max depth to not do super long recall chain
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
impl<T: Clone> Node<T>{
    pub fn new(IN_val: T, IN_depth: u16, IN_maxDepth: u16) -> Self{
        Self{
            val: IN_val,
            depth: IN_depth,
            maxDepth: IN_maxDepth,
            nodes: Vec::new()
        }
    }

    pub fn addNode(&mut self, IN_val: T) -> usize{
        self.nodes.push(Self::new(IN_val, self.depth + 1, self.maxDepth));
        self.nodes.len() - 1
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