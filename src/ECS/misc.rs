use super::*;

use vars::*;

#[derive(Clone, Copy)]
pub struct gmGenIndex<T>{
    pub id: gmID,
    pub gen: gmGen,
    entry: Option<T>
}
impl<T> gmGenIndex<T>{
    pub fn new(IN_id: gmID, IN_entry: Option<T>) -> Self{
        Self{
            id: IN_id,
            gen: 0,
            entry: IN_entry,
        }
    }
    pub fn unset(&mut self){
        self.entry = None;
        self.gen += 1;
    }
}
impl<T> Deref for gmGenIndex<T>{
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}
impl<T> DerefMut for gmGenIndex<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry
    }
}