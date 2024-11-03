#![allow(dead_code)]
use std::any::Any;
use std::collections::{BTreeMap, HashMap};

use super::*;

mod comp;
mod storage;
mod system;
mod vars;
mod world;
mod resource;

use comp::*;
use storage::*;
use system::*;
use vars::*;
use world::*;
use resource::*;

pub struct gmDispatcher{
    systems: HashMap<&'static str, usize>,
    stages: Vec<gmDispatchStage>
}
impl gmDispatcher{
    pub fn new() -> Self{
        Self{
            systems: HashMap::new(),
            // Singular item to avoid checks for empty vec later
            stages: Vec::from([gmDispatchStage::new()])
        }
    }
    pub fn withSys<T>(mut self, IN_depends: &[&'static str]) -> Self where T: for<'a> gmSystem<'a> + 'static{
        self.addSys::<T>(IN_depends);
        self
    }
    pub fn addSys<T>(&mut self, IN_depends: &[&'static str]) where T: for<'a> gmSystem<'a> + 'static{
        // Check if the system is registered already
        if self.systems.contains_key(T::SYS_ID()){
            return
        }

        let mut w_nextStage: usize = 0;
        
        'CHECKSTAGE:{
            // Exit early if there's no dependencies
            if IN_depends.is_empty(){
                break 'CHECKSTAGE;
            }

            for DEPEND in IN_depends.iter(){
                // Check if such dependency exists
                if let Some(STAGEID) = self.systems.get(DEPEND){
                    // If it's later in processing than latest recorded stage, update it
                    if *STAGEID > w_nextStage{
                        w_nextStage = *STAGEID + 1
                    }
                }
            }
        }

        // Check if the desired stage exists
        if let Some(STAGE) = self.stages.get_mut(w_nextStage){
            STAGE.addSys::<T>();
            return
        }
        // If not, add a new stage with the system
        self.addStage(gmDispatchStage::new().withSys::<T>());

    }
    pub fn addStage(&mut self, IN_stage: gmDispatchStage){
        self.stages.push(IN_stage);
    }
    pub fn dispatch(&mut self, IN_world: &mut gmWorld){
        for STAGE in self.stages.iter_mut(){
            STAGE.dispatch(IN_world);
        }
    }
}

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

pub struct gmObjBuilder<'a>{
    pub gmObjID: gmID,
    pub worldRef: &'a mut gmWorld
}
impl gmObjBuilder<'_>{
    pub fn addComp<T>(self, IN_comp: T) -> Self where T:gmComp{
        // I gotta deal with this
        self.worldRef.fetchMut::<T>().insert(self.gmObjID, IN_comp);
        self
    }
}

mod tests{
    use event::*;
    use time::Duration;

    use super::*;

    pub fn main(){
        let mut world = gmWorld::new();

        world.registerComp::<gmComp_Health>();
        world.registerComp::<gmComp_Pos>();
        
        world.registerRes::<gmRes_deltaT>();
        world.registerRes::<gmRes_PInput>();
        
        world.createGmObj()
            .addComp::<gmComp_Health>(gmComp_Health{val: 100})
            .addComp::<gmComp_Pos>(gmComp_Pos{x: 0, y: 0});

        let mut dispatcher = gmDispatcher::new()
            .withSys::<gmSys_input>(&[])
            .withSys::<gmSys_HP>(&[]);

        dispatcher.dispatch(&mut world);

        world.deleteGmObj(0);
    }

    pub struct gmComp_Health{
        pub val: gmID
    }
    impl gmComp for gmComp_Health{
        type COMP_STORAGE = denseVecStorage<Self>;
        fn COMP_ID() -> &'static str {
            "gmComp_Health"
        }
    }

    pub struct gmComp_Pos{
        pub x: usize,
        pub y: usize
    }
    impl gmComp for gmComp_Pos{
        type COMP_STORAGE = denseVecStorage<Self>;
        fn COMP_ID() -> &'static str {
            "gmComp_Pos"
        }
    }

    pub struct gmSys_HP{}
    impl<'a> gmSystem<'a> for gmSys_HP{
        type sysData = gmSysData_HP<'a>;

        fn new() -> Self {
            Self{}
        }

        fn SYS_ID() -> &'static str {
            "gmSys_HP"
        }

        fn execute(&mut self, IN_data: Self::sysData) {
            for COMP_HP in IN_data.comp_HP.inner.iter_mut(){
                if COMP_HP.val.val > 0{
                    COMP_HP.val.val -= 1
                }
            }
        }
    }
    pub struct gmSysData_HP<'a>{
        pub comp_HP: &'a mut <gmComp_Health as ECS::gmComp>::COMP_STORAGE
    }
    impl<'a> gmSystemData<'a> for gmSysData_HP<'a>{
        fn fetch(IN_world: &'a mut gmWorld) -> Self {
            Self{
                comp_HP: IN_world.fetchMut::<gmComp_Health>()
            }
        }
    }

    pub struct gmSys_input{}
    impl<'a> gmSystem<'a> for gmSys_input{
        type sysData = gmSysData_Input<'a>;

        fn new() -> Self {
            Self{}
        }

        fn SYS_ID() -> &'static str {
            "gmSys_input"
        }

        fn execute(&mut self, IN_data: Self::sysData) {
            if !poll(Duration::from_secs(0)).unwrap(){
                IN_data.res_Input.res = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE);
                return
            }

            if let Event::Key(KEY) = read().unwrap(){
                IN_data.res_Input.res = KEY;
                return
            }
        }
    }
    pub struct gmSysData_Input<'a>{
        pub res_Input: &'a mut gmRes_PInput
    }
    impl<'a> gmSystemData<'a> for gmSysData_Input<'a>{
        fn fetch(IN_world: &'a mut gmWorld) -> Self {
            Self{
                res_Input: IN_world.fetchResMut::<gmRes_PInput>()
            }
        }
    }

    pub struct denseVecStorage<T>{
        pub proxyMap: HashMap<gmID, usize>,
        pub inner: Vec<denseVecEntry<T>>
    }
    impl<T: 'static> gmStorage<T> for denseVecStorage<T>{

        fn new() -> Self {
            Self{
                proxyMap: HashMap::new(),
                inner: Vec::new()
            }
        }

        fn insert(&mut self, IN_id: gmID, IN_item: T) {
            if self.proxyMap.contains_key(&IN_id){return}

            self.proxyMap.insert(IN_id, self.inner.len()); // Vec length is always the next free index
            self.inner.push(
                denseVecEntry{
                    id: IN_id,
                    val: IN_item,
                }
            );
        }
    
        fn remove(&mut self, IN_id: gmID) -> Option<T> {
            
            if let Some(INDEX) = self.proxyMap.remove(&IN_id){

                if INDEX == self.inner.len() - 1{
                    return Some(self.inner.pop().unwrap().val)
                }

                *self.proxyMap.get_mut(&self.inner.last().unwrap().id).unwrap() = INDEX;

                return Some(self.inner.swap_remove(INDEX).val);
            }
            None

        }
    }
    pub struct denseVecEntry<T>{
        id: gmID,
        val: T
    }

    pub struct gmRes_deltaT{
        res: Duration
    }
    impl gmRes for gmRes_deltaT{
        fn new() -> Self {
            Self{
                res: Duration::from_secs(0)
            }
        }
        fn RES_ID() -> &'static str {
            "gmRes_deltaT"
        }
    }

    pub struct gmRes_PInput{
        res: KeyEvent
    }
    impl gmRes for gmRes_PInput{
        fn new() -> Self {
            Self{
                res: KeyEvent{
                    code: KeyCode::Null,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    state: KeyEventState::NONE,
                }
            }
        }
        fn RES_ID() -> &'static str {
            "gmResPInput"
        }
    }
}