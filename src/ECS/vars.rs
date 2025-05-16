use std::cell::{Ref, RefMut};

use super::*;

use events::gmEvent;
use fetch::*;

pub struct gmWorld_EVENTMAP{
    activeBuffer: bool,
    inner: HashMap<&'static str, (Rc<RefCell<dyn eventQueue>>, Rc<RefCell<dyn eventQueue>>)>,
}
impl gmWorld_EVENTMAP{
    pub fn new() -> Self{
        Self{
            activeBuffer: false,
            inner: HashMap::new(),
        }
    }

    pub fn registerEvent<T>(&mut self) where T: gmEvent + 'static{
        use std::collections::hash_map::Entry;
        match self.inner.entry(T::EVENT_ID()){
            Entry::Occupied(_) => panic!("ERROR: Attempted to override an existing event: {}", T::EVENT_ID()),
            Entry::Vacant(ENTRY) => ENTRY.insert((Rc::new(RefCell::new(Vec::<T>::new())), Rc::new(RefCell::new(Vec::<T>::new())))), // This is a mess
        };
    }

    pub fn unRegisterEvent<T>(&mut self) where T: gmEvent + 'static{
        self.inner.remove(T::EVENT_ID());
    }

    fn getBuffer<T>(&self, IN_buffer: bool) -> &Rc<RefCell<dyn eventQueue>> where T: gmEvent +'static{
        self.getBufferByID(T::EVENT_ID(), IN_buffer)
    }

    fn getBufferByID(&self, IN_id: &str, IN_buffer: bool) -> &Rc<RefCell<dyn eventQueue>>{
        match self.inner.get(IN_id){
            Some(EVQUE) => {
                if IN_buffer{
                    return &EVQUE.1
                }
                return &EVQUE.0
            },
            None => panic!("ERROR: Tried to fetch an unregistered event: {}", IN_id),
        }
    }

    pub fn getEventReader<'a, T>(&'a self) -> EventReader<'a, T> where T: gmEvent + 'static{
        EventReader::new(
            Ref::map(self.getBuffer::<T>(self.activeBuffer).as_ref().borrow(), |idkfa| idkfa.downcast_ref::<T>())
        )
    }

    pub fn getEventWriter<'a, T>(&'a self) -> EventWriter<'a, T> where T: gmEvent + 'static{
        EventWriter::new(
            RefMut::map(self.getBuffer::<T>(!self.activeBuffer).as_ref().borrow_mut(), |idkfa| idkfa.downcast_mut::<T>())
        )
    }

    pub fn switchBuffer(&mut self){
        self.activeBuffer = !self.activeBuffer
    }

    pub fn switchNClear(&mut self){
        for (_, EVENT) in self.inner.iter_mut(){
            if self.activeBuffer{
                EVENT.1.as_ref().borrow_mut().clear();
            }else{
                EVENT.0.as_ref().borrow_mut().clear();
            }
        }
        self.switchBuffer();
    }
}

pub trait eventQueue{
    fn clear(&mut self);
}
impl<T: gmEvent> eventQueue for Vec<T>{
    fn clear(&mut self) {
        self.clear();
    }
}
impl dyn eventQueue{
    pub fn downcast_ref<T: gmEvent>(&self) -> &Vec<T>{
        unsafe {&*(self as *const dyn eventQueue as *const Vec<T>)}
    }
    pub fn downcast_mut<T: gmEvent>(&mut self) -> &mut Vec<T>{
        unsafe {&mut *(self as *mut dyn eventQueue as *mut Vec<T>)}
    }
}