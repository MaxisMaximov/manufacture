use super::*;

pub struct res_PInput{
    pub res: KeyEvent
}
impl gmRes for res_PInput{
    fn new() -> Self {
        Self{
            res: KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
        }
    }

    fn RES_ID() -> &'static str {
        "res_PInput"
    }
}

pub struct res_DeltaT{
    pub res: Duration
}
impl gmRes for res_DeltaT{
    fn new() -> Self {
        Self{
            res: Duration::from_secs(0)
        }
    }

    fn RES_ID() -> &'static str {
        "res_DeltaT"
    }
}

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

pub struct res_PID{
    pub res: HashMap<gmID, gmID> // PID, gmObjID
}
impl gmRes for res_PID{
    fn new() -> Self {
        Self{
            res: HashMap::new()
        }
    }

    fn RES_ID() -> &'static str {
        "res_PID"
    }
}

pub struct res_GridWorld{
    
}
pub struct GridWorldChunk{

}
pub struct GridWorldTile{

}