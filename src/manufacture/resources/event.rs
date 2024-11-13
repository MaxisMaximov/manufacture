use super::*;

pub struct res_Events{
    pub activeBuffer: bool,
    pub inner: HashMap<&'static str, (Box<dyn Any>, Box<dyn Any>)>,
}
impl res_Events{

    pub fn registerEvent<T>(&mut self) where T: gmEvent + 'static{
        self.inner.entry(T::EVENT_ID()).or_insert((Box::new(Vec::<T>::new()), Box::new(Vec::<T>::new())));
    }

    pub fn unRegisterEvent<T>(&mut self) where T: gmEvent + 'static{
        self.inner.remove(T::EVENT_ID());
    }

    fn getActiveBuffer<T>(&mut self) -> &mut Vec<T> where T: gmEvent + 'static{
        let BUFFERS = self.inner.get_mut(T::EVENT_ID()).unwrap();

        if self.activeBuffer{BUFFERS.1.downcast_mut::<Vec<T>>().unwrap()}
        else{BUFFERS.0.downcast_mut::<Vec<T>>().unwrap()}
    }

    fn getAlternateBuffer<T>(&mut self) -> &mut Vec<T> where T: gmEvent + 'static{
        let BUFFERS = self.inner.get_mut(T::EVENT_ID()).unwrap();

        if self.activeBuffer {BUFFERS.1.downcast_mut::<Vec<T>>().unwrap()}
        else{BUFFERS.0.downcast_mut::<Vec<T>>().unwrap()}
    }

    pub fn read<T>(&mut self) -> &Vec<T> where T: gmEvent + 'static{
        self.getActiveBuffer::<T>()
    }

    pub fn push<T>(&mut self, IN_event: T) where T: gmEvent + 'static{
        self.getAlternateBuffer().push(IN_event)
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