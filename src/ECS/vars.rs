use std::cell::{RefMut, Ref};
use std::ops::{Deref, DerefMut};

use super::*;

use misc::gmGenIndex;
use comp::gmComp;
use events::gmEvent;
use resource::gmRes;

pub struct Fetch<'a, T: gmComp>{
    pub inner: Ref<'a, T::COMP_STORAGE>
}
impl<'a, T: gmComp> Deref for Fetch<'a, T>{
    type Target = Ref<'a, T::COMP_STORAGE>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
pub struct FetchMut<'a, T: gmComp>{
    pub inner: RefMut<'a, T::COMP_STORAGE>
}
impl<'a, T: gmComp> Deref for FetchMut<'a, T>{
    type Target = RefMut<'a, T::COMP_STORAGE>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T: gmComp> DerefMut for FetchMut<'a, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct FetchRes<'a, T: gmRes>{
    pub inner: Ref<'a, T>
}
impl<'a, T: gmRes> Deref for FetchRes<'a, T>{
    type Target = Ref<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
pub struct FetchResMut<'a, T: gmRes>{
    pub inner: RefMut<'a, T>
}
impl<'a, T: gmRes> Deref for FetchResMut<'a, T>{
    type Target = RefMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T: gmRes> DerefMut for FetchResMut<'a, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


pub struct EventReader<'a, T: gmEvent>{
    pub inner: Ref<'a, Vec<T>>
}
impl<'a, T: gmEvent> Deref for EventReader<'a, T>{
    type Target = Ref<'a, Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct EventWriter<'a, T: gmEvent>{
    pub inner: RefMut<'a, Vec<T>>
}
impl<'a, T: gmEvent> Deref for EventWriter<'a, T>{
    type Target = RefMut<'a, Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T: gmEvent> DerefMut for EventWriter<'a, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub type gmWorld_COMPMAP = HashMap<&'static str, Rc<dyn Any>>;
pub type gmWorld_RESMAP = HashMap<&'static str, Rc<dyn Any>>;
pub type gmObj = gmGenIndex<()>;
pub type gmID = u16;
pub type gmGen = u16; // There is no way you can even remotely get to 32kth generation -- Consider it a gift

pub struct gmWorld_EVENTMAP{
    pub activeBuffer: bool,
    pub inner: HashMap<&'static str, (Rc<dyn Any>, Rc<dyn Any>)>,
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

    fn getBuffer<T>(&self, IN_buffer: bool) -> &Rc<dyn Any> where T: gmEvent +'static{
        let idkfa_eventQueue = self.inner.get(T::EVENT_ID()).unwrap();
        if IN_buffer{
            return &idkfa_eventQueue.1
        }
        &idkfa_eventQueue.0
        
    }

    pub fn getEventReader<'a, T>(&'a self) -> EventReader<'a, T> where T: gmEvent + 'static{
        EventReader{
            inner: self.getBuffer::<T>(self.activeBuffer).as_ref().downcast_ref::<RefCell<Vec<T>>>().unwrap().borrow()
        }
    }

    pub fn getEventWriter<'a, T>(&'a self) -> EventWriter<'a, T> where T: gmEvent + 'static{
        EventWriter{
            inner: self.getBuffer::<T>(!self.activeBuffer).as_ref().downcast_ref::<RefCell<Vec<T>>>().unwrap().borrow_mut()
        }
    }

    pub fn switchBuffer(&mut self){
        self.activeBuffer = !self.activeBuffer
    }
}