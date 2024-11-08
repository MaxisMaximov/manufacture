use super::*;

pub struct res_PInput{
    res: KeyEvent
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
    res: Duration
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
    activeBuffer: bool,
    bufferA: HashMap<&'static str, Box<dyn Any>>,
    bufferB: HashMap<&'static str, Box<dyn Any>>
}
impl res_Events{
    pub fn pushA<T>(&mut self, IN_event: T) where T: gmEvent + 'static{
        self.bufferA.get_mut(T::EVENT_ID()).unwrap().downcast_mut::<Vec<T>>().unwrap().push(IN_event);
    }
    pub fn readA<T>(&mut self) -> &Vec<T> where T: gmEvent + 'static{
        self.bufferA.get(T::EVENT_ID()).unwrap().downcast_ref::<Vec<T>>().unwrap()
    }

    pub fn pushB<T>(&mut self, IN_event: T) where T: gmEvent + 'static{
        self.bufferB.get_mut(T::EVENT_ID()).unwrap().downcast_mut::<Vec<T>>().unwrap().push(IN_event);
    }
    pub fn readB<T>(&mut self) -> &Vec<T> where T: gmEvent + 'static{
        self.bufferB.get(T::EVENT_ID()).unwrap().downcast_ref::<Vec<T>>().unwrap()
    }
}
impl gmRes for res_Events{
    fn new() -> Self {
        Self{
            activeBuffer: false,
            bufferA: HashMap::new(),
            bufferB: HashMap::new()
        }
    }

    fn RES_ID() -> &'static str {
        "res_Event"
    }
}
pub trait gmEvent{
    fn EVENT_ID() -> &'static str;
}