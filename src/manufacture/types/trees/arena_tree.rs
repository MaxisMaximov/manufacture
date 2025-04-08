use std::collections::{BTreeMap, BTreeSet};

/// Arena-allocated tree
/// 
/// Every Node is distinguished by it's ID and Hash
/// 
/// Access to Nodes is granted through Handles, allowing for in-place manipulation of the node  
/// A "reference" to the node is granted through Tokens
/// 
/// Tokens are a way for other structs to "memorize" a node  
/// Use `get_handle_from_token` to get a handle to the Node and modify it
/// 
/// Cursor allows for controlled traversal of the tree
pub struct ArenaTree<T>{
    nodes: BTreeMap<usize, ArenaNode<T>>,
    // A Hack to not paste a super long chunk of code each time
    // And technically the root _is_ a node, you just can't access it
    root: ArenaNode<()>,
    next_free: BTreeSet<usize>
}
impl<T> ArenaTree<T>{
    /// Creates a new empty, tree
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

    /// Insert a Node into the tree and assign it to a parent
    /// 
    /// A Parent of `None` makes the Node a Root Node
    /// 
    /// This is typically used to append nodes to existing nodes 
    /// or start new roots within the tree
    /// 
    /// Returns a Handle to the node for immediate in-place 
    /// manipulation, like with `.with` method
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
    /// Insert a Node into the tree as *n*-th child of an existing Node
    /// 
    /// A Parent of `None` makes the Node a Root Node
    /// 
    /// This is typically used to insert nodes inbetween existing Nodes
    /// or between Root Nodes of the tree
    /// 
    /// Returns a Handle to the node for immediate in-place 
    /// manipulation, like with `.with` method
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
    /// Removes the Node and all of the node's Subnodes
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
    /// Removes the Node and makes all the Node's Subnodes new Root Nodes
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
    /// Removes the Node and reassigns all of the Node's Subnodes to the Node's parent
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
    /// Get a reference to the value stored in a Node
    pub fn get(&self, Index: &usize) -> Option<&T>{
        Some(&self.nodes.get(Index)?.val)
    }
    /// Get a mutable reference to the value stored in a Node
    pub fn get_mut(&mut self, Index: &usize) -> Option<&mut T>{
        Some(&mut self.nodes.get_mut(Index)?.val)
    }
    /// Get a reference to a Node
    /// 
    /// This is typically used to get just the node's data without needing a Handle
    fn get_node(&self, Index: &usize) -> Option<&ArenaNode<T>>{
        self.nodes.get(Index)
    }
    /// Get a mutable reference to a Node
    /// 
    /// ## WARNING
    /// Do not modify any field apart from Value
    fn get_node_mut(&mut self, Index: &usize) -> Option<&mut ArenaNode<T>>{
        self.nodes.get_mut(Index)
    }

    /// Get a reference to the Node's parent
    pub fn get_parent(&self, Index: &usize) -> Option<&ArenaNode<T>>{
        let parent_index = self.nodes.get(Index)?.parent?;
        self.nodes.get(&parent_index)
    }
    /// Get a mutable reference to a Node's parent
    /// 
    /// ## WARNING
    /// Do not modify any field apart from Value
    fn get_parent_mut(&mut self, Index: &usize) -> Option<&mut ArenaNode<T>>{
        let parent_index = self.nodes.get(Index)?.parent?;
        self.nodes.get_mut(&parent_index)
    }

    /// Get a reference to the Node's Left Sibling, also called Previous Sibling
    pub fn get_left_sibling(&self, Index: &usize) -> Option<&ArenaNode<T>>{
        let index = self.get_parent(Index)?.child_before(Index)?;
        self.nodes.get(&index)
    }
    /// Get a mutable reference to a Node's Left Sibling
    /// 
    /// ## WARNING
    /// Do not modify any field apart from Value
    fn get_left_sibling_mut(&mut self, Index: &usize) -> Option<&mut ArenaNode<T>>{
        let index = self.get_parent(Index)?.child_before(Index)?;
        self.nodes.get_mut(&index)
    }
    /// Get a reference to the Node's LefRight Sibling, also called Next Sibling
    pub fn get_right_sibling(&self, Index: &usize) -> Option<&ArenaNode<T>>{
        let index = self.get_parent(Index)?.child_after(Index)?;
        self.nodes.get(&index)
    }
    /// Get a mutable reference to a Node's Right Sibling
    /// 
    /// ## WARNING
    /// Do not modify any field apart from Value
    fn get_right_sibling_mut(&mut self, Index: &usize) -> Option<&mut ArenaNode<T>>{
        let index = self.get_parent(Index)?.child_after(Index)?;
        self.nodes.get_mut(&index)
    }

    /// Get a Handle from a Token for in-place manipulation
    /// 
    /// Typically used to manipulate only the nodes that the program is keeping track of
    pub fn get_handle_from_token(&mut self, Token: &mut ArenaToken) -> Option<ArenaHandle<'_, T>>{
        // Early return if token is laready marked as invalid
        if !Token.valid{
            return None
        }
        match self.nodes.get(&Token.node) {
            // If Hashes ain't same it's an invalid Token
            Some(node) if node.hash == Token.hash => {
                Some(ArenaHandle::new(self, Token.node))
            }
            // Either the node doesn't exist or the Hashes ain't same
            _ => {Token.valid = false; None},
        }
    }
    /// Get a Handle for a current Node
    /// 
    /// Typically used for in-place manipulation of the node, similar to `Entry` API in `HashMap` and `BTreeMap`
    pub fn get_handle(&mut self, Index: &usize) -> Option<ArenaHandle<'_, T>>{
        if self.nodes.contains_key(Index){
            Some(ArenaHandle::new(self, *Index))
        }else{
            None
        }
    }
    /// Get a Token 'referencing' a current Node
    /// 
    /// Typically used to get a Handle to the referenced Node through `get_handle_from_token`
    pub fn get_token(&self, Index: &usize) -> Option<ArenaToken>{
        if let Some(node) = self.nodes.get(Index){
            Some(ArenaToken::new_from_node(node))
        }else{
            None
        }
    }
    /// Detach a Node from it's parent
    /// 
    /// Typically used to sepparate Nodes into Root Subtrees for organization purposes
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
    /// Attach a Node to another Node
    /// 
    /// Typically used to join Root Subtrees for organization purposes
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
    /// Attach a Node to another Node as n-th Child
    /// 
    /// Typically used to insert Root Subtrees into other Trees at speciic position for organization purposes
    /// 
    /// If the Position is outside of the Parent Node's Children list, it gets attached as last
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
    /// Change a Node's Parent
    /// 
    /// Typically used to move a Subtree into a different Tree for organization purposes
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
    /// Change a Node's position within it's Parent's Children list
    /// 
    /// Typically used to reorder Child Nodes for organization purposes
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
    /// Traverse the Tree using Iterator with Depth-First Traversal, starting from the Tree's Root
    pub fn traverse(&self) -> Traverse<'_, T>{
        Traverse::new(self)
    }
    /// Traverse the Tree using Iterator with Depth-First Traversal, starting from a specific Node
    /// 
    /// If the Node doesn't exist, the Iterator is empty and doesn't do anything
    pub fn traverse_from(&self, StartNode: &usize) -> Traverse<'_, T>{
        Traverse::new_from_node(self, StartNode)
    }
    /// Traverse the Tree using a controllable Cursor
    pub fn cursor(&mut self) -> ArenaCursor<'_, T>{
        ArenaCursor::new(self)
    }
    /// Traverse the Tree using a controllable Cursor
    /// 
    /// If the Node doesn't exist, the Cursor defaults to the Root level
    pub fn cursor_from(&mut self, StartNode: &usize) -> ArenaCursor<'_, T>{
        ArenaCursor::new_from_node(self, *StartNode)
    }
    /// Perform operations on the Tree immediately after creating it
    /// 
    /// This method is typically used to initialize Nodes right away
    pub fn with_map<F: FnOnce(&mut ArenaTree<T>)>(mut self, f: F) -> Self{
        f(&mut self);
        self
    }
}

/// # Safe Interface to the node
/// Allows for in-place manipulation of a Node, including moving it in the Tree, adding Subnodes and operations on Self
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
    /// Get a token for the current Node
    pub fn get_token(&self) -> ArenaToken{
        ArenaToken::new(self.node, self.tree_ref.get_node(&self.node).unwrap().hash)
    }
    /// Add a Subnode to the current Node
    pub fn add_child(&mut self, Val: T) -> ArenaHandle<'_, T>{
        self.tree_ref.insert(Val, Some(self.node))
    }
    /// Add a Subnode to the current Node at n-th position
    pub fn add_child_nth(&mut self, Val: T, Position: usize) -> ArenaHandle<'_, T>{
        self.tree_ref.insert_nth(Val, Some(self.node), Position)
    }
    /// Remove a Subnode from the current Node
    /// 
    /// Equivalent to the Tree's `remove` method, but checks if the Node is a Subnode of the current Node
    /// 
    /// Searches for the given Node's Index and removes it from the Tree
    pub fn remove_child(&mut self, Index: &usize){
        if self.tree_ref.get_node(&self.node).unwrap().has_child(Index){
            self.tree_ref.remove(Index);
        }
    }
    /// Soft Remove a Subnode from the current Node
    /// 
    /// Equivalent to the Tree's `soft_remove` method, but checks if the Node is a Subnode of the current Node
    /// 
    /// Searches for the given Node's Index and soft removes it from the Tree
    pub fn soft_remove_child(&mut self, Index: &usize){
        if self.tree_ref.get_node(&self.node).unwrap().has_child(Index){
            self.tree_ref.soft_remove(Index);
        }
    }
    /// Dissolve a Subnode of the current Node
    ///
    /// Equivalent to the Tree's `dissolve` method, but checks if the Node is a Subnode of the current Node
    /// 
    /// Searches for the given Node's Index and dissolves it from the Tree
    pub fn dissolve_child(&mut self, Index: &usize){
        if self.tree_ref.get_node(&self.node).unwrap().has_child(Index){
            self.tree_ref.dissolve(Index);
        }
    }
    /// Change a Subnode's position within the current Node
    pub fn reorder_child(&mut self, Child: &usize, NewPosition: usize){
        if self.tree_ref.get_node(&self.node).unwrap().has_child(Child){
            self.tree_ref.get_node_mut(&self.node).unwrap().change_child_order(Child, NewPosition);
        }
    }
    /// Remove the current Node
    pub fn remove_self(self){
        self.tree_ref.remove(&self.node);
    }
    /// Soft Remove the current Node, making all Subnodes new Root Nodes
    pub fn soft_remove_self(self){
        self.tree_ref.soft_remove(&self.node);
    }
    /// Dissolve the current Node, assigning all Subnodes to the Node's Parent
    pub fn dissolve_self(self){
        self.tree_ref.dissolve(&self.node);
    }
    /// Change order of current Node within it's Parent
    pub fn reorder_self(&mut self, NewPosition: usize){
        self.tree_ref.change_order_position(&self.node, NewPosition);
    }
    /// Perform operations using the current Node
    /// 
    /// This is typically used to daisy-chain Node Creation for easier Tree initialization
    pub fn map<F: FnOnce(&mut ArenaHandle<'_, T>)>(&mut self, f: F){
        f(self)
    }
    /// Get current Node's Value
    /// 
    /// This is the only field that can be mutated
    pub fn val(&mut self) -> &mut T{
        self.tree_ref.get_mut(&self.node).unwrap()
    }
    /// Get current Node's Hash
    pub fn hash(&self) -> u32{
        self.tree_ref.get_node(&self.node).unwrap().hash
    }
    /// Get current Node's Parent index
    pub fn parent(&self) -> &Option<usize>{
        &self.tree_ref.get_node(&self.node).unwrap().parent
    }
    /// Get current Node's Children indexes
    pub fn children(&self) -> &[usize]{
        &self.tree_ref.get_node(&self.node).unwrap().children
    }
    /// Convert the Handle into a Cursor for controlled Traversal
    pub fn into_cursor(self) -> ArenaCursor<'a, T>{
        ArenaCursor::new_from_node(self.tree_ref, self.node)
    }
}

/// # A 'reference' to an Arena Tree Node
/// Holds the node's Index and Hash to avoid collisions with Nodes at same position
/// 
/// Tokens whose Nodes no longer exist are invalid  
/// This is checked through the Hash value
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
    /// Append a Child Node at the end of the Node
    fn push_child(&mut self, Child: usize){
        self.children.push(Child);
    }
    /// Insert a Child Node at position within the Node
    fn insert_child(&mut self, Child: usize, Position: usize){
        if Position < self.children.len(){
            self.children.insert(Position, Child);
        }else{
            // Failsafe to avoid panic
            self.children.push(Child);
        }
    }
    /// Change the order of a child within the Node
    fn change_child_order(&mut self, Child: &usize, NewPosition: usize){
        if self.remove_child(Child){
            self.insert_child(*Child, NewPosition);
        }
    }
    /// Remove an Index from the Node's children
    /// 
    /// This method searches for the Index and removes it from the list
    ///
    /// Returns True if it had the node
    fn remove_child(&mut self, Child: &usize) -> bool{
        if let Some(index) = self.children.iter().position(|x| x == Child){
            self.children.remove(index);
            true
        }else{
            false
        }
    }
    /// Remove a Child Node at a position
    ///
    /// Returns True if it had the node
    fn remove_nth_child(&mut self, Index: &usize) -> bool{
        if Index < &self.children.len(){
            self.children.remove(*Index);
            true
        }else{
            false
        }
    }
    /// Check if the Node contains the Child index
    fn has_child(&self, Child: &usize) -> bool{
        self.children.iter().position(|x| x == Child).is_some()
    }
    /// Get the index of the left Sibling of a Child
    fn child_before(&self, Child: &usize) -> Option<usize>{
        match self.children.iter().position(|x| x == Child){
            Some(index) if index > 0 => Some(self.children[index - 1]),
            _ => None,
        }
    }
    /// Get the index of the Right Sibling of a Child
    fn child_after(&self, Child: &usize) -> Option<usize>{
        match self.children.iter().position(|x| x == Child){
            Some(index) => self.children.get(index + 1).copied(),
            _ => None,
        }
    }
    /// Get the index of the first Child of the Node
    fn first_child(&self) -> Option<usize>{
        self.children.first().cloned()
    }
    /// Get the index of the last Child of the Node
    fn last_child(&self) -> Option<usize>{
        self.children.last().cloned()
    }
}


/// # Depth-First Traversal Iterator
/// Traverses the Tree Depth-First and revisits the nodes on it's way up
/// 
/// Every iteration returns a tuple `(Traverse Level, Node Reference)`
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

            let node = self.tree_ref.get_node(&frame.0).unwrap();

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

/// # Controlled Tree Traversal
/// 
/// Allows for free movement within the tree for reading values
/// 
/// For manipulation of the Nodes, use `into_handle` to get a Handle
pub struct ArenaCursor<'a, T>{
    tree_ref: &'a mut ArenaTree<T>,
    node: Option<usize>,
}
impl<'a, T> ArenaCursor<'a, T>{
    fn new(TreeRef: &'a mut ArenaTree<T>) -> Self{
        Self{
            tree_ref: TreeRef,
            node: None,
        }
    }
    fn new_from_node(TreeRef: &'a mut ArenaTree<T>, StartNode: usize) -> Self{
        Self{
            tree_ref: TreeRef,
            node: Some(StartNode),
        }
    }
    /// Move up a level to the Node's parent
    pub fn move_up(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            self.node = self.tree_ref.get_node(&index).unwrap().parent
        }
        self
    }
    /// Move down a level to the Node's first Child
    pub fn move_down(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(next_node) = self.tree_ref.get_node(&index).unwrap().children.get(0){
                self.node = Some(*next_node)
            }
        }
        self
    }
    /// Move left on the same level to the Node's Previous Sibling
    pub fn move_left(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(next_node) = self.tree_ref.get_parent(&index).unwrap().child_before(&index){
                self.node = Some(next_node)
            }
        }
        self
    }
    /// Move right on the same level to the Node's Next Sibling
    pub fn move_right(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(next_node) = self.tree_ref.get_parent(&index).unwrap().child_after(&index){
                self.node = Some(next_node)
            }
        }
        self
    }
    /// Move to the Node's first Sibling
    pub fn move_to_first(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(first_node) = self.tree_ref.get_parent(&index).unwrap().first_child(){
                self.node = Some(first_node)
            }
        }
        self
    }
    /// Move to the Node's last Sibling
    pub fn move_to_last(&mut self) -> &mut Self{
        if let Some(index) = self.node{
            if let Some(last_node) = self.tree_ref.get_parent(&index).unwrap().last_child(){
                self.node = Some(last_node)
            }
        }
        self
    }
    /// Move to a specific Node within the tree
    /// 
    /// It will remain on the current Node if the requested Node doesn't exist
    pub fn move_to_node(&mut self, Node: &usize) -> &mut Self{
        if self.tree_ref.get_node(Node).is_some(){
            self.node = Some(*Node)
        }
        self
    }
    /// Move to the Root level of the Tree
    pub fn move_to_root(&mut self) -> &mut Self{
        self.node = None;
        self
    }
    /// Get a token for the current Node
    pub fn get_token(&mut self) -> Option<ArenaToken>{
        if let Some(index) = self.node{
            let node = self.tree_ref.get_node(&index).unwrap();
            Some(ArenaToken::new(index, node.hash))
        }else{
            None
        }
    }
    /// Get the Value of the current Node
    pub fn val(&mut self) -> Option<&mut T>{
        if let Some(index) = self.node{
            Some(&mut self.tree_ref.get_node_mut(&index).unwrap().val)
        }else{
            None
        }
    }
    /// Get the Hash of the current Node
    pub fn hash(&self) -> Option<&u32>{
        if let Some(index) = self.node{
            Some(&self.tree_ref.get_node(&index).unwrap().hash)
        }else{
            None
        }
    }
    /// Get the Parent Index of the current Node
    pub fn parent(&self) -> Option<&Option<usize>>{
        if let Some(index) = self.node{
            Some(&self.tree_ref.get_node(&index).unwrap().parent)
        }else{
            None
        }
    }
    /// Get the Child Indexes of the current Node
    pub fn children(&self) -> Option<&[usize]>{
        if let Some(index) = self.node{
            Some(&self.tree_ref.get_node(&index).unwrap().children)
        }else{
            None
        }
    }
    /// Convert the Cursor into a Handle for in-place manipulation
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
