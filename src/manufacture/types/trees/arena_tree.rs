use std::collections::{BTreeMap, BTreeSet};

pub struct ArenaTree<T>{
    nodes: BTreeMap<usize, ArenaNode<T>>,
    // A Hack to not paste a super long chunk of code each time
    // And technically the root _is_ a node, you just can't access it
    root: ArenaNode<()>,
    next_free: BTreeSet<usize>
}
impl<T> ArenaTree<T>{
    pub fn new() -> Self{
        Self{
            nodes: BTreeMap::new(),
            root: ArenaNode::new((), 0, None),
            next_free: BTreeSet::new(),
        }
    }


    fn free_index(&mut self, Index: &usize) -> Option<ArenaNode<T>>{
        match self.nodes.remove(Index){
            Some(node) => {
                self.next_free.insert(*Index);
                Some(node)
            },
            None => None,
        }
    }
    fn free_index_unchecked(&mut self, Index: &usize) -> ArenaNode<T>{
        self.next_free.insert(*Index);
        self.nodes.remove(Index).unwrap()
    }


    pub fn insert(&mut self, Val: T, Parent: Option<usize>) -> ArenaHandle<'_, T>{
        // Find next available index
        // Length of the Node Map is always the next available index if the `next_free` is empty
        let next_index = self.next_free.pop_first().unwrap_or(self.nodes.len());

        let mut final_parent = None;
        // Check if parent exists
        // None is auto failsafe
        if let Some(index) = Parent{
            if let Some(parent_node) = self.nodes.get_mut(&index){
                parent_node.push_child(next_index);
                final_parent = Parent;
            }
        }else{
            self.root.push_child(next_index);
        }

        self.nodes.insert(
            next_index,
            ArenaNode::new(Val, next_index, final_parent)
        );

        ArenaHandle::new(self, next_index)
    }
    pub fn insert_nth(&mut self, Val: T, Parent: Option<usize>, Position: usize) -> ArenaHandle<'_, T>{
        let next_index = self.next_free.pop_first().unwrap_or(self.nodes.len());

        let mut final_parent = None;
        if let Some(index) = Parent{
            if let Some(parent_node) = self.nodes.get_mut(&index){
                parent_node.insert_child(next_index, Position);
                final_parent = Parent;
            }
        }else{
            self.root.insert_child(next_index, Position);
        }

        self.nodes.insert(
            next_index,
            ArenaNode::new(Val, next_index, final_parent)
        );

        ArenaHandle::new(self, next_index)
    }
    // Removes the node and all the subnodes
    pub fn remove(&mut self, Index: &usize) -> Option<ArenaNode<T>>{
        if let Some(node) = self.free_index(Index){

            // Update parent if exists
            if let Some(index) = node.parent{
                // Unwrap: The Parent is always valid
                let parent_node = self.nodes.get_mut(&index).unwrap();

                parent_node.remove_child(Index);
            }else{
                self.root.remove_child(Index);
            }

            // Remove subnodes
            // Cloning the Vec to return the node unscathed
            let mut to_remove = node.children.clone();
            while let Some(index) = to_remove.pop(){
                let subnode = self.free_index_unchecked(&index);

                // Extend to keep iterating
                // It's the reason I use While loop instead of For loop
                to_remove.extend(subnode.children.iter());
            }

            Some(node)
        }else{None}
    }
    // Removes the nde and puts subnodes at root level
    pub fn soft_remove(&mut self, Index: &usize) -> Option<ArenaNode<T>>{
        if let Some(node) = self.free_index(Index){

            // Update parent
            if let Some(index) = node.parent{
                // Unwrap: The Parent is always valid
                self.nodes.get_mut(&index).unwrap().remove_child(Index);
            }

            // Reassign subnodes
            for index in node.children.iter(){
                self.nodes.get_mut(index).unwrap().parent = None;
                self.root.push_child(*index);
            }

            Some(node)
        }else{None}
    }
    // Removes the node and assigns all subnodes to the parent node
    pub fn dissolve(&mut self, Index: &usize) -> Option<ArenaNode<T>>{
        if let Some(node) = self.free_index(Index){

            // Update parent with new subnodes if exists
            if let Some(index) = node.parent{
                // Unwrap: The Parent is always valid
                let parent_node = self.nodes.get_mut(&index).unwrap();

                parent_node.remove_child(Index);

                for index in node.children.iter(){ 
                    parent_node.push_child(*index);
                }
            }

            // Reassign subnodes
            for index in node.children.iter(){
                let subnode = self.nodes.get_mut(&index).unwrap();
                subnode.parent = node.parent
            }

            Some(node)
        }else{None}
    }


    pub fn get(&self, Index: &usize) -> Option<&ArenaNode<T>>{
        self.nodes.get(Index)
    }
    pub fn get_mut(&mut self, Index: &usize) -> Option<&mut ArenaNode<T>>{
        self.nodes.get_mut(Index)
    }

    pub fn get_parent(&self, Index: &usize) -> Option<&ArenaNode<T>>{
        let parent_index = self.nodes.get(Index)?.parent?;
        self.nodes.get(&parent_index)
    }
    pub fn get_parent_mut(&mut self, Index: &usize) -> Option<&mut ArenaNode<T>>{
        let parent_index = self.nodes.get(Index)?.parent?;
        self.nodes.get_mut(&parent_index)
    }

    pub fn get_left_sibling(&self, Index: &usize) -> Option<&ArenaNode<T>>{
        let index = self.get_parent(Index)?.child_before(Index)?;
        self.nodes.get(&index)
    }
    pub fn get_left_sibling_mut(&mut self, Index: &usize) -> Option<&mut ArenaNode<T>>{
        let index = self.get_parent(Index)?.child_before(Index)?;
        self.nodes.get_mut(&index)
    }
    pub fn get_right_sibling(&self, Index: &usize) -> Option<&ArenaNode<T>>{
        let index = self.get_parent(Index)?.child_after(Index)?;
        self.nodes.get(&index)
    }
    pub fn get_right_sibling_mut(&mut self, Index: &usize) -> Option<&mut ArenaNode<T>>{
        let index = self.get_parent(Index)?.child_after(Index)?;
        self.nodes.get_mut(&index)
    }

    pub fn get_handle_from_token(&mut self, Token: &mut ArenaToken) -> Option<ArenaHandle<'_, T>>{
        // Early return if token is laready marked as invalid
        if !Token.valid{
            return None
        }
        match self.nodes.get(&Token.node) {
            // If hashes ain't same it's an invalid Token
            Some(node) if node.hash == Token.hash => {
                Some(ArenaHandle::new(self, Token.node))
            }
            // Either the node doesn't exist or the Hashes ain't same
            _ => {Token.invalidate(); None},
        }
    }
    pub fn get_handle(&mut self, Index: &usize) -> Option<ArenaHandle<'_, T>>{
        if self.nodes.contains_key(Index){
            Some(ArenaHandle::new(self, *Index))
        }else{
            None
        }
    }
    pub fn get_token(&self, Index: &usize) -> Option<ArenaToken>{
        if let Some(node) = self.nodes.get(Index){
            Some(ArenaToken::new_from_node(node))
        }else{
            None
        }
    }

    pub fn clear_parent(&mut self, Index: &usize){
        if let Some((index, mut node)) = self.nodes.remove_entry(Index){ // Quick yoink

            if let Some(parent_index) = node.parent.take(){ // Take autosets the old parent to None, so that's pretty handy
                // Unwrap: Parent is always valid
                self.root.push_child(index);
                self.nodes.get_mut(&parent_index).unwrap().remove_child(&index);
            }

            self.nodes.insert(index, node);
        }
    }

    pub fn assign_parent(&mut self, Index: &usize, NewParent: usize){
        if let Some((index, mut node)) = self.nodes.remove_entry(Index){

            // Only do so if the node has no parent
            if node.parent.is_none(){
                if let Some(new_parent) = self.nodes.get_mut(&NewParent){
                    self.root.remove_child(&index);
                    node.parent = Some(NewParent);
                    new_parent.push_child(index);
                }
            }

            self.nodes.insert(index, node);
        }
    }

    pub fn assign_parent_as_nth(&mut self, Index: &usize, NewParent: usize, Position: usize){
        if let Some((index, mut node)) = self.nodes.remove_entry(Index){

            // Only do so if the node has no parent
            if node.parent.is_none(){
                if let Some(new_parent) = self.nodes.get_mut(&NewParent){
                    self.root.remove_child(&index);
                    node.parent = Some(NewParent);
                    new_parent.push_child(index);
                }
            }

            self.nodes.insert(index, node);
        }
    }

    pub fn change_parent(&mut self, Index: &usize, NewParent: usize){
        if let Some((index, mut node)) = self.nodes.remove_entry(Index){
            
            // Tear the edge between the node and old parent
            if let Some(parent_index) = node.parent.take(){
                self.nodes.get_mut(&parent_index).unwrap().remove_child(&index);
            }
            
            // Create a new edge between the node and new parent
            if let Some(new_parent) = self.nodes.get_mut(&NewParent){
                node.parent = Some(NewParent);
                new_parent.push_child(index);
            }

            self.nodes.insert(index, node);
        }
    }

    pub fn change_order_position(&mut self, Index: &usize, NewPosition: usize){
        if let Some((index, node)) = self.nodes.remove_entry(Index){
                match node.parent{
                    Some(p_index) => {
                        self.nodes.get_mut(&p_index).unwrap().change_child_order(Index, NewPosition);
                    },
                    None => {
                        self.root.change_child_order(Index, NewPosition);
                    },
                }
            self.nodes.insert(index, node);
        }
    }

    pub fn traverse(&self) -> Traverse<'_, T>{
        Traverse::new(self)
    }

    pub fn traverse_from(&self, StartNode: &usize) -> Traverse<'_, T>{
        Traverse::new_from_node(self, StartNode)
    }

    pub fn with_nodes<F: FnOnce(&mut ArenaTree<T>)>(mut self, f: F) -> Self{
        f(&mut self);
        self
    }
}

// Interface to the node
pub struct ArenaHandle<'a, T>{
    tree_ref: &'a mut ArenaTree<T>,
    node: usize,
}
impl<'a, T> ArenaHandle<'a, T>{
    fn new(TreeRef: &'a mut ArenaTree<T>, Node: usize) -> Self{
        Self{
            tree_ref: TreeRef,
            node: Node,
        }
    }

    pub fn get_token(&self) -> ArenaToken{
        ArenaToken::new(self.node, self.tree_ref.get(&self.node).unwrap().hash)
    }

    pub fn add_child(&mut self, Val: T) -> ArenaHandle<'_, T>{
        self.tree_ref.insert(Val, Some(self.node))
    }
    pub fn add_child_nth(&mut self, Val: T, Position: usize) -> ArenaHandle<'_, T>{
        self.tree_ref.insert_nth(Val, Some(self.node), Position)
    }
    pub fn remove_child(&mut self, Index: &usize){
        if self.tree_ref.get(&self.node).unwrap().has_child(Index){
            self.tree_ref.remove(Index);
        }
    }
    pub fn soft_remove_child(&mut self, Index: &usize){
        if self.tree_ref.get(&self.node).unwrap().has_child(Index){
            self.tree_ref.soft_remove(Index);
        }
    }
    pub fn dissolve_child(&mut self, Index: &usize){
        if self.tree_ref.get(&self.node).unwrap().has_child(Index){
            self.tree_ref.dissolve(Index);
        }
    }
    pub fn reorder_child(&mut self, Child: &usize, NewPosition: usize){
        if self.tree_ref.get(&self.node).unwrap().has_child(Child){
            self.tree_ref.get_mut(&self.node).unwrap().change_child_order(Child, NewPosition);
        }
    }

    pub fn remove_self(self){
        self.tree_ref.remove(&self.node);
    }
    pub fn soft_remove_self(self){
        self.tree_ref.soft_remove(&self.node);
    }
    pub fn dissolve_self(self){
        self.tree_ref.dissolve(&self.node);
    }
    pub fn reorder_self(&mut self, NewPosition: usize){
        self.tree_ref.change_order_position(&self.node, NewPosition);
    }

    pub fn map<F: FnOnce(&mut ArenaHandle<'_, T>)>(&mut self, f: F){
        f(self)
    }

    pub fn into_cursor(self) -> ArenaCursor<'a, T>{
        ArenaCursor::new_from_node(self.tree_ref, self.node)
    }
}


pub struct ArenaToken{
    node: usize,
    hash: u32,
    valid: bool
}
impl ArenaToken{
    fn new(Index: usize, Hash: u32) -> Self{
        Self{
            node: Index,
            hash: Hash,
            valid: true
        }
    }
    fn new_from_node<T>(Node: &ArenaNode<T>) -> Self{
        Self{
            node: Node.id,
            hash: Node.hash,
            valid: true,
        }
    }
    fn invalidate(&mut self){
        self.valid = false
    }
}


pub struct ArenaNode<T>{
    val: T,
    id: usize,
    hash: u32, // I know it's technically not a 'hash' but it sounds cool so peck it
    parent: Option<usize>,
    children: Vec<usize>
}
impl<T> ArenaNode<T>{
    fn new(Val: T, Id: usize, Parent: Option<usize>) -> Self{
        Self{
            val: Val,
            id: Id,
            hash: rand::random(),
            parent: Parent,
            children: Vec::new(),
        }
    }

    fn push_child(&mut self, Child: usize){
        self.children.push(Child);
    }
    fn insert_child(&mut self, Child: usize, Position: usize){
        if Position < self.children.len(){
            self.children.insert(Position, Child);
        }else{
            // Failsafe to avoid panic
            self.children.push(Child);
        }
    }
    fn change_child_order(&mut self, Child: &usize, NewPosition: usize){
        if self.remove_child(Child){
            self.insert_child(*Child, NewPosition);
        }
    }
    fn remove_child(&mut self, Child: &usize) -> bool{
        if let Some(index) = self.children.iter().position(|x| x == Child){
            self.children.remove(index);
            true
        }else{
            false
        }
    }
    fn has_child(&self, Child: &usize) -> bool{
        self.children.iter().position(|x| x == Child).is_some()
    }

    fn child_before(&self, Child: &usize) -> Option<usize>{
        match self.children.iter().position(|x| x == Child){
            Some(index) if index > 0 => Some(self.children[index - 1]),
            _ => None,
        }
    }
    fn child_after(&self, Child: &usize) -> Option<usize>{
        match self.children.iter().position(|x| x == Child){
            Some(index) => self.children.get(index + 1).copied(),
            _ => None,
        }
    }
    fn first_child(&self) -> Option<usize>{
        self.children.first().cloned()
    }
    fn last_child(&self) -> Option<usize>{
        self.children.last().cloned()
    }
}


// Traverses Depth-first and visits nodes on the way back up
pub struct Traverse<'a, T>{
    tree_ref: &'a ArenaTree<T>,
    stack: Vec<(usize, bool)>, // (nodeID, visited)
    next_down: bool
}
impl<'a, T> Traverse<'a, T>{
    fn new(TreeRef: &'a ArenaTree<T>) -> Self{
        Self{
            tree_ref: TreeRef,
            stack: {
                let mut idkfa = Vec::new();
                for index in TreeRef.root.children.iter(){
                    idkfa.push((*index, false));
                }
                idkfa
            },
            next_down: false,
        }
    }
    fn new_from_node(TreeRef: &'a ArenaTree<T>, Node: &usize) -> Self{
        Self{
            tree_ref: TreeRef,
            stack: Vec::from([(*Node, false)]),
            next_down: false,
        }
    }
}
impl<'a, T> Iterator for Traverse<'a, T>{
    type Item = (TraverseLevel, &'a ArenaNode<T>);

    fn next(&mut self) -> Option<Self::Item>{
        if let Some(mut frame) = self.stack.pop(){

            let node = self.tree_ref.get(&frame.0).unwrap();

            // If we got to a visited node, we went up a level
            if frame.1{
                self.next_down = false;
                return Some((TraverseLevel::Up, node))
            }

            // Bandaid fix to make sure we know we traversed down
            let Out = 
                if self.next_down{
                    self.next_down = false;
                    (TraverseLevel::Down, node)
                }else{
                    (TraverseLevel::Same, node)
                };

            // Check if node has subnodes
            if !node.children.is_empty(){
                // Put the frame back into the stack as visited
                frame.1 = true;
                self.stack.push(frame);

                // Reverse to make first child at the top of the stack
                for index in node.children.iter().rev(){
                    self.stack.push((*index, false));
                }
                // At this point the next node we enter will be down a level
                self.next_down = true;
            };
            
            Some(Out)
        // .pop() returned nothing, the stack is empty, we can leave
        }else{None}
    }
}

pub struct ArenaCursor<'a, T>{
    tree_ref: &'a mut ArenaTree<T>,
    node: Option<usize>,
}
impl<'a, T> ArenaCursor<'a, T>{
    pub fn new(TreeRef: &'a mut ArenaTree<T>) -> Self{
        Self{
            tree_ref: TreeRef,
            node: None,
        }
    }
    pub fn new_from_node(TreeRef: &'a mut ArenaTree<T>, StartNode: usize) -> Self{
        Self{
            tree_ref: TreeRef,
            node: Some(StartNode),
        }
    }

    pub fn move_up(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            self.node = self.tree_ref.get(&index).unwrap().parent
        }
        self
    }
    pub fn move_down(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(next_node) = self.tree_ref.get(&index).unwrap().children.get(0){
                self.node = Some(*next_node)
            }
        }
        self
    }
    pub fn move_left(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(next_node) = self.tree_ref.get_parent(&index).unwrap().child_before(&index){
                self.node = Some(next_node)
            }
        }
        self
    }
    pub fn move_right(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(next_node) = self.tree_ref.get_parent(&index).unwrap().child_after(&index){
                self.node = Some(next_node)
            }
        }
        self
    }
    pub fn move_to_first(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(first_node) = self.tree_ref.get_parent(&index).unwrap().first_child(){
                self.node = Some(first_node)
            }
        }
        self
    }
    pub fn move_to_last(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(last_node) = self.tree_ref.get_parent(&index).unwrap().last_child(){
                self.node = Some(last_node)
            }
        }
        self
    }

    pub fn move_to_node(&mut self, Node: &usize) -> &mut Self{
        if self.tree_ref.get(Node).is_some(){
            self.node = Some(*Node)
        }
        self
    }
    pub fn move_to_root(&mut self) -> &mut Self{
        self.node = None;
        self
    }

    pub fn get_token(&mut self) -> Option<ArenaToken>{
        if let Some(index) = self.node{
            let node = self.tree_ref.get(&index).unwrap();
            Some(ArenaToken::new(index, node.hash))
        }else{
            None
        }
    }

    pub fn get_val(&mut self) -> Option<&mut T>{
        if let Some(index) = self.node{
            Some(&mut self.tree_ref.get_mut(&index).unwrap().val)
        }else{
            None
        }
    }

    pub fn into_handle(self) -> Result<ArenaHandle<'a, T>, Self>{
        if let Some(index) = self.node{
            Ok(ArenaHandle::new(self.tree_ref, index))
        }else{
            Err(self)
        }
    }
}
pub enum TraverseLevel{
    Up,
    Same,
    Down
}
