use super::*;

pub struct comp_inventory{
    pub inner: Vec<item>,
    pub maxSize: usize,
    pub index: usize
}
impl comp_inventory{
    pub fn new(IN_maxSize: usize) -> Self{
        Self{
            inner: Vec::with_capacity(IN_maxSize),
            maxSize: IN_maxSize,
            index: 0,
        }
    }
    pub fn invOp(&mut self, IN_op: invOps){
        match IN_op {
            invOps::select(OPMODE) => {
                // If it's empty just set it to 0
                if self.inner.is_empty(){
                    self.index = 0;
                    return;
                }

                // Forward & Check overflow
                if OPMODE && self.index < (self.inner.len() - 1){
                    self.index += 1;
                    return;
                }
                // Backward & Check underflow
                if !OPMODE && self.index > 0{
                    self.index -= 1
                }
            },
            invOps::modify(OPMODE) => {
                // Ignore if it's all empty
                if self.inner.is_empty(){
                    return;
                }
                // Increase & Check overflow
                if OPMODE && self.inner[self.index].id < 255{
                    self.inner[self.index].id += 1;
                    return;
                }
                // Decrease & Check underflow
                if !OPMODE && self.inner[self.index].id > 0{
                    self.inner[self.index].id -= 1;
                    return;
                }
            },
            invOps::addDel(OPMODE) => {
                // Add & Check overflow
                if OPMODE && self.inner.len() < self.maxSize{
                    self.inner.insert(self.index, item::new());
                    return
                }

                // Remove ONLY if there is something in the inventory
                if !OPMODE && !self.inner.is_empty(){
                    self.inner.remove(self.index);
                    self.invOp(invOps::select(false));
                    return;
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum invOps{
    /// 1 - Forward | 0 - Backward
    select(bool),
    /// 1 - Increment | 0 - Decrement
    modify(bool),
    /// 1 - Add | 0 - Delete
    addDel(bool)
}

pub static item_hashmap: Lazy<HashMap<u8, &str>> = Lazy::new(|| HashMap::from(
    [
        (0, "INVALID"),
        (1, "Ingot"),
        (2, "Ore"),
        (3, "Component Kit")
    ]
));