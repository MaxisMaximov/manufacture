use super::*;

use vars::*;

pub trait gmStorage<T>: Any{
    fn new() -> Self where Self: Sized;
    fn insert(&mut self, IN_id: gmID, IN_item: T);
    fn remove(&mut self, IN_id: gmID) -> Option<T>;
}
pub trait gmStorageDrop: Any{
    fn drop(&mut self, IN_id: gmID);
}
impl<T: 'static> gmStorageDrop for dyn gmStorage<T>{
    fn drop(&mut self, IN_id: gmID) {
        self.remove(IN_id);
    }
}


pub struct denseVecStorage<T>{
    pub proxyMap: HashMap<gmID, usize>,
    pub inner: Vec<denseVecEntry<T>>
}
impl<T: 'static> gmStorage<T> for denseVecStorage<T>{

    fn new() -> Self {
        Self{
            proxyMap: HashMap::new(),
            inner: Vec::new()
        }
    }

    fn insert(&mut self, IN_id: gmID, IN_item: T) {
        if self.proxyMap.contains_key(&IN_id){return}

        self.proxyMap.insert(IN_id, self.inner.len()); // Vec length is always the next free index
        self.inner.push(
            denseVecEntry{
                id: IN_id,
                val: IN_item,
            }
        );
    }

    fn remove(&mut self, IN_id: gmID) -> Option<T> {
        
        if let Some(INDEX) = self.proxyMap.remove(&IN_id){

            if INDEX == self.inner.len() - 1{
                return Some(self.inner.pop().unwrap().val)
            }

            *self.proxyMap.get_mut(&self.inner.last().unwrap().id).unwrap() = INDEX;

            return Some(self.inner.swap_remove(INDEX).val);
        }
        None

    }
}
pub struct denseVecEntry<T>{
    pub id: gmID,
    pub val: T
}