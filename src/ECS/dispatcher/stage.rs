use super::*;

use world::*;
use system::*;

pub struct gmDispatchStage{
    pub systems: HashMap<&'static str, ()>,
    pub inner: Vec<Box<dyn for<'a> gmSysRun<'a>>>
}
impl gmDispatchStage{
    pub fn new() -> Self{
        Self{
            systems: HashMap::new(),
            inner: Vec::new()
        }
    }
    pub fn withSys<T>(mut self) -> Self where T: for<'a> gmSystem<'a> + 'static{
        self.addSys::<T>();
        self
    }
    pub fn addSys<T>(&mut self) where T: for<'a> gmSystem<'a> + 'static{
        if self.systems.contains_key(T::SYS_ID()){return}
        
        self.systems.insert(T::SYS_ID(), ());
        self.inner.push(Box::new(T::new()));
    }
    pub fn checkSys<T>(&self) -> bool where T: for<'a> gmSystem<'a>{
        self.checkSysID(T::SYS_ID())
    }
    pub fn checkSysID(&self, IN_id: &'static str) -> bool{
        match self.systems.get(IN_id){
            Some(_) => true,
            None => false,
        }
    }
    pub fn dispatch(&mut self, IN_world: &mut gmWorld){
        for SYS in self.inner.iter_mut(){
            SYS.executeNow(IN_world);
        }
    }
}