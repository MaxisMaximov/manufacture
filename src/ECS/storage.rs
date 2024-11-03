use super::*;

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