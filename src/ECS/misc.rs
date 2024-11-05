use super::*;

use vars::*;

#[derive(Clone, Copy)]
pub struct gmGenIndex<T: Sized>{
    pub id: gmID,
    pub gen: gmGen,
    pub entry: Option<T>
}
impl<T: Sized> gmGenIndex<T>{
    pub fn new(IN_id: gmID, IN_entry: Option<T>) -> Self{
        Self{
            id: IN_id,
            gen: 0,
            entry: IN_entry,
        }
    }
    pub fn set(&mut self, IN_entry: T){
        self.entry = Some(IN_entry);
    }
    pub fn unset(&mut self){
        self.entry = None;
        self.gen += 1;
    }
}