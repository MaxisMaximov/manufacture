use std::cell::{Ref, RefMut};

use super::*;

use commands::gmCommand;
use misc::gmGenIndex;
use events::gmEvent;
use fetch::*;
use storage::gmStorageDrop;

pub type gmWorld_COMPMAP = HashMap<&'static str, Rc<RefCell<dyn gmStorageDrop>>>;
pub type gmWorld_RESMAP = HashMap<&'static str, Rc<RefCell<dyn Any>>>;
pub type gmWorld_CMDQUEUE = Vec<Box<dyn gmCommand>>;
pub type gmObj = gmGenIndex<()>;
pub type gmID = u16;
pub type gmGen = u16; // There is no way you can even remotely get to 32kth generation -- Consider it a gift

pub struct gmWorld_EVENTMAP{
    pub activeBuffer: bool,
    pub inner: HashMap<&'static str, (Rc<RefCell<dyn eventQueue>>, Rc<RefCell<dyn eventQueue>>)>,
}
impl gmWorld_EVENTMAP{
    pub fn new() -> Self{
        Self{
            activeBuffer: false,
            inner: HashMap::new(),
        }
    }

    pub fn registerEvent<T>(&mut self) where T: gmEvent + 'static{
        self.inner
            .entry(T::EVENT_ID())
            .or_insert((Rc::new(RefCell::new(Vec::<T>::new())), Rc::new(RefCell::new(Vec::<T>::new()))));
    }

    pub fn unRegisterEvent<T>(&mut self) where T: gmEvent + 'static{
        self.inner.remove(T::EVENT_ID());
    }

    fn getBuffer<T>(&self, IN_buffer: bool) -> &Rc<RefCell<dyn eventQueue>> where T: gmEvent +'static{
        self.getBufferByID(T::EVENT_ID(), IN_buffer)
    }

    fn getBufferByID(&self, IN_id: &str, IN_buffer: bool) -> &Rc<RefCell<dyn eventQueue>>{
        let idkfa_eventQueue = self.inner.get(IN_id).unwrap();
        if IN_buffer{
            return &idkfa_eventQueue.1
        }
        &idkfa_eventQueue.0
    }

    pub fn getEventReader<'a, T>(&'a self) -> EventReader<'a, T> where T: gmEvent + 'static{
        EventReader{
            data: Ref::map(self.getBuffer::<T>(self.activeBuffer).as_ref().borrow(), |idkfa| idkfa.downcast_ref::<T>())
        }
    }

    pub fn getEventWriter<'a, T>(&'a self) -> EventWriter<'a, T> where T: gmEvent + 'static{
        EventWriter{
            data: RefMut::map(self.getBuffer::<T>(!self.activeBuffer).as_ref().borrow_mut(), |idkfa| idkfa.downcast_mut::<T>())
        }
    }

    pub fn switchBuffer(&mut self){
        self.activeBuffer = !self.activeBuffer
    }

    pub fn switchNClear(&mut self){
        for (_, EVENT) in self.inner.iter_mut(){
            if self.activeBuffer{
                EVENT.1.as_ref().borrow_mut().clear();
                continue
            }
            EVENT.0.as_ref().borrow_mut().clear();
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