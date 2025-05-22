use std::{cell::{Ref, RefMut}, collections::{HashSet, VecDeque}};

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

/// # Event Map
/// A Double Hashmap Buffer queue of Events
/// 
/// Maintains a Registry of Events to prevent illegal overrides and reading/writing non-existent Events
/// 
/// Buffers switch at the end of every Tick, clearing the previously Read-Only buffer
pub struct EventMap{
    registry: HashSet<&'static str>,
    active_buffer: HashMap<&'static str, RefCell<Box<dyn Any>>>,
    alt_buffer: HashMap<&'static str, RefCell<Box<dyn Any>>>,
}
impl EventMap{
    /// Create a new, empty EventMap
    pub fn new() -> Self{
        Self{
            registry: HashSet::new(),
            active_buffer: HashMap::new(),
            alt_buffer: HashMap::new(),
        }
    }

    /// Register an event
    pub fn register<T: gmEvent>(&mut self){
        if self.registry.contains(T::EVENT_ID()){
            // Events CANNOT share IDs because systems expect a specific type
            // This ain't OOP, we can't replace one struct with another and expect it to go the same
            panic!("ERROR: Conflicting Event IDs: {}", T::EVENT_ID())
        }
        self.registry.insert(T::EVENT_ID());
    }
    /// Deregister an event
    /// 
    /// This also clears the respective Event's Queues from both buffers
    pub fn deregister<T: gmEvent>(&mut self){
        self.registry.remove(T::EVENT_ID());
        // Remove those events from the Map as they're no longer valid
        self.active_buffer.remove(T::EVENT_ID());
        self.alt_buffer.remove(T::EVENT_ID());
    }

    fn swap_buffers(&mut self){
        // Clear active buffer to (kinda) free up memory
        self.active_buffer.clear();
        std::mem::swap(&mut self.active_buffer, &mut self.alt_buffer);
    }
    /// Get a Reader for an Event
    /// 
    /// Panics if the requested Event is not registered
    pub fn get_reader<'a, T: gmEvent + 'static>(&'a mut self) -> EventReader<'a, T>{
        // Check if the Event is valid
        if !self.registry.contains(T::EVENT_ID()){
            panic!("ERROR: Attempted to fetch unregistered event: {}", T::EVENT_ID())
        }

        // If we don't have this key in buffer yet, initialize it
        // This is a hacky workaround to return a "valid" `EventReader`
        // even if the queue for that event is empty
        if !self.alt_buffer.contains_key(T::EVENT_ID()){
            self.alt_buffer.insert(T::EVENT_ID(), RefCell::new(Box::new(VecDeque::<T>::new())));
        }

        // We have checks for valid ID and a backup Queue, so we can safely unwrap
        let queue = self.alt_buffer.get(T::EVENT_ID()).unwrap();

        EventReader::new(
            Ref::map(
                queue.borrow(), 
                |x| x.downcast_ref::<VecDeque<T>>().unwrap())
        )
    }
    /// Get a Writer for an Event
    /// 
    /// Panics if the requested Event is not registered
    pub fn get_writer<'a, T: gmEvent + 'static>(&'a mut self) -> EventWriter<'a, T>{
        // CHeck if the Event is valid
        if !self.registry.contains(T::EVENT_ID()){
            panic!("ERROR: Attempted to fetch unregistered event: {}", T::EVENT_ID())
        }

        // If there's a valid event but no queue, set up a new one
        if !self.active_buffer.contains_key(T::EVENT_ID()){
            self.active_buffer.insert(T::EVENT_ID(), RefCell::new(Box::new(VecDeque::<T>::new())));
        }

        // We have checks for valid ID and a backup Queue, so we can safely unwrap
        let queue = self.active_buffer.get(T::EVENT_ID()).unwrap();

        EventWriter::new(
            RefMut::map(
                queue.borrow_mut(),
                |x| x.downcast_mut::<VecDeque<T>>().unwrap())
        )
    }
}