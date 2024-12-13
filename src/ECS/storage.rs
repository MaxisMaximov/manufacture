use super::*;

use comp::gmComp;
use vars::*;

pub trait gmStorage<T: gmComp>: Any{
    fn new() -> Self where Self: Sized;
    fn get(&self, IN_id: &gmID) -> &T;
    fn get_mut(&mut self, IN_id: &gmID) -> &mut T;
    fn insert(&mut self, IN_id: gmID, IN_item: T);
    fn remove(&mut self, IN_id: &gmID) -> Option<T>;
}
pub trait gmStorageDrop: Any{
    fn drop(&mut self, IN_id: &gmID);
}

impl dyn gmStorageDrop{
    pub fn downcast<T: gmComp>(&self) -> &gmStorageContainer<T>{
        unsafe {&*(self as *const dyn gmStorageDrop as *const gmStorageContainer<T>)}
    }
    pub fn downcast_mut<T: gmComp>(&mut self) -> &mut gmStorageContainer<T>{
        unsafe {&mut *(self as *mut dyn gmStorageDrop as *mut gmStorageContainer<T>)}
    }
}

pub struct gmStorageContainer<T:gmComp>{
    pub inner: T::COMP_STORAGE
}
impl<T: gmComp + 'static> gmStorageDrop for gmStorageContainer<T>{
    fn drop(&mut self, IN_id: &gmID) {
        self.inner.remove(IN_id);
    }
}


pub struct denseVecStorage<T>{
    pub proxyMap: HashMap<gmID, usize>,
    pub inner: Vec<denseVecEntry<T>>
}
impl<T: gmComp + 'static> gmStorage<T> for denseVecStorage<T>{

    fn new() -> Self {
        Self{
            proxyMap: HashMap::new(),
            inner: Vec::new()
        }
    }

    fn get(&self, IN_id: &gmID) -> &T {
        &self.inner.get(*self.proxyMap.get(&IN_id).unwrap()).unwrap().val
    }

    fn get_mut(&mut self, IN_id: &gmID) -> &mut T {
        &mut self.inner.get_mut(*self.proxyMap.get(&IN_id).unwrap()).unwrap().val
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

    fn remove(&mut self, IN_id: &gmID) -> Option<T> {
        
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