use super::*;

use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

pub struct res_Events{
    pub activeBuffer: bool,
    pub inner: HashMap<&'static str, (Rc<Box<dyn Any>>, Rc<Box<dyn Any>>)>,
}
impl res_Events{

    pub fn registerEvent<T>(&mut self) where T: gmEvent + 'static{
        self.inner
            .entry(T::EVENT_ID())
            .or_insert((Rc::new(Box::new(RefCell::new(Vec::<T>::new()))), Rc::new(Box::new(RefCell::new(Vec::<T>::new())))));
    }

    pub fn unRegisterEvent<T>(&mut self) where T: gmEvent + 'static{
        self.inner.remove(T::EVENT_ID());
    }

    fn getBuffer<T>(&self, IN_buffer: bool) -> &Rc<Box<dyn Any>> where T: gmEvent +'static{
        let idkfa_eventQueue = self.inner.get(T::EVENT_ID()).unwrap();
        if IN_buffer{
            return &idkfa_eventQueue.1
        }
        &idkfa_eventQueue.0
        
    }

    pub fn getEventReader<'a, T>(&'a self) -> FetchEvent<'a, T> where T: gmEvent + 'static{
        FetchEvent{
            inner: self.getBuffer::<T>(self.activeBuffer).as_ref().downcast_ref::<RefCell<Vec<T>>>().unwrap().borrow()
        }
    }

    pub fn getEventWriter<'a, T>(&'a self) -> FetchEventMut<'a, T> where T: gmEvent + 'static{
        FetchEventMut{
            inner: self.getBuffer::<T>(!self.activeBuffer).as_ref().downcast_ref::<RefCell<Vec<T>>>().unwrap().borrow_mut()
        }
    }

    pub fn switchBuffer(&mut self){
        self.activeBuffer = !self.activeBuffer
    }
}
impl gmRes for res_Events{
    fn new() -> Self {
        Self{
            activeBuffer: false,
            inner: HashMap::new()
        }
    }

    fn RES_ID() -> &'static str {
        "res_Event"
    }
}
pub trait gmEvent{
    fn EVENT_ID() -> &'static str;
}

pub struct event_TileChange{
    pub coords: Vector2,
    pub newTile: u8
}
impl gmEvent for event_TileChange{
    fn EVENT_ID() -> &'static str {
        "event_TileChange"
    }
}

pub struct event_BatchTileChange{
    pub from: Vector2,
    pub to: Vector2,
    pub newTile: u8
}
impl gmEvent for event_BatchTileChange{
    fn EVENT_ID() -> &'static str {
        "event_BatchTileChange"
    }
}