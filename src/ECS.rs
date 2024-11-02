#![allow(dead_code)]
use std::any::Any;
use std::collections::{BTreeMap, HashMap};

use super::*;

pub trait gmComp: Any + Sized{
    type COMP_STORAGE: gmStorage<Self>;
    fn COMP_ID() -> &'static str;
}

pub trait gmRes: Any{
    fn new() -> Self;
    fn RES_ID() -> &'static str;
}

pub struct gmWorld{
    pub gmObjs: gmObjStorage,
    pub components: gmWorld_COMPMAP,
    pub resources: gmWorld_RESMAP,
}
impl gmWorld{

    pub fn new() -> Self{
        Self{
            gmObjs: gmObjStorage::new(),
            components: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    pub fn fetch<T>(&self) -> &T::COMP_STORAGE where T: gmComp + 'static{
        self.components.get(T::COMP_ID()).unwrap().downcast_ref::<T::COMP_STORAGE>().unwrap()
    }
    pub fn fetchMut<T>(&mut self) -> &mut T::COMP_STORAGE where T: gmComp + 'static{
        self.components.get_mut(T::COMP_ID()).unwrap().downcast_mut::<T::COMP_STORAGE>().unwrap()
    }

    pub fn fetchRes<T>(&self) -> &T where T: gmRes + 'static{
        self.resources.get(T::RES_ID()).unwrap().downcast_ref::<T>().unwrap()
    }
    pub fn fetchResMut<T>(&mut self) -> &mut T where T: gmRes + 'static{
        self.resources.get_mut(T::RES_ID()).unwrap().downcast_mut::<T>().unwrap()
    }

    pub fn registerComp<T>(&mut self) where T: gmComp + 'static{
        self.components.insert(
            T::COMP_ID(),
            Box::new(T::COMP_STORAGE::new())
        );
    }
    pub fn unRegisterComp<T>(&mut self) where T: gmComp + 'static{
        self.components.remove(T::COMP_ID());
    }

    pub fn registerRes<T>(&mut self) where T: gmRes + 'static{
        self.resources.insert(T::RES_ID(), Box::new(T::new()));
    }
    pub fn unRegisterRes<T>(&mut self) where T: gmRes + 'static{
        self.resources.remove(T::RES_ID());
    }

    pub fn createGmObj(&mut self) -> gmObjBuilder{
        gmObjBuilder{
            gmObjID: self.gmObjs.insertNextFree(),
            worldRef: self,
        }
    }

    pub fn deleteGmObj(&mut self, IN_id: gmID){
        self.gmObjs.remove(IN_id);
            for COMP in self.components.values_mut(){
                COMP.downcast_mut::<&mut dyn gmStorageDrop>().unwrap().drop(IN_id);
            }
    }
}

pub trait gmSystem<'a>{
    type sysData: gmSystemData<'a>;
    fn new() -> Self;
    fn SYS_ID() -> &'static str;
    fn execute(&mut self, IN_data: Self::sysData);
}

pub trait gmSysRun<'a>{
    fn executeNow(&mut self, IN_world: &'a mut gmWorld);
}
impl<'a, T> gmSysRun<'a> for T where T:gmSystem<'a>{
    fn executeNow(&mut self, IN_world: &'a mut gmWorld) {
        self.execute(T::sysData::fetch(IN_world));
    }
}

pub trait gmSystemData<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self;
}

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

pub trait gmStorage<T>: Any{
    fn new() -> Self where Self: Sized;
    fn insert(&mut self, IN_id: gmID, IN_item: T);
    fn remove(&mut self, IN_id: gmID) -> Option<T>;
}
pub trait gmStorageDrop: Any{
    fn drop(&mut self, IN_id: gmID);
}
impl<T: 'static> gmStorageDrop for dyn gmStorage<T>{
    fn drop(&mut self, IN_id: gmID) {
        self.remove(IN_id);
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

pub struct gmObjStorage{
    pub gmObjMap: HashMap<gmID, gmObj>,
    pub nextFree: BTreeMap<gmID, ()>,
}
impl gmObjStorage{
    pub fn new() -> Self{
        Self{
            gmObjMap: HashMap::new(),
            nextFree: BTreeMap::new()
        }
    }

    pub fn insert(&mut self, IN_id: gmID){
        self.gmObjMap.entry(IN_id)
            .and_modify(|ENTRY| ENTRY.set(()))
            .or_insert(gmObj::new(IN_id, Some(())));
    }

    pub fn insertNextFree(&mut self) -> gmID{
        let w_nextIndex: gmID = self.nextFree.pop_first().unwrap_or((self.gmObjMap.len() as gmID, ())).0;

        self.insert(w_nextIndex);

        return w_nextIndex
    }

    pub fn remove(&mut self, IN_id: gmID){
        if let Some(ENTRY) = self.gmObjMap.get_mut(&IN_id){
            ENTRY.unset();
            self.nextFree.insert(IN_id, ());
        }
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

pub type gmWorld_COMPMAP = HashMap<&'static str, Box<dyn Any>>;
pub type gmWorld_RESMAP = HashMap<&'static str, Box<dyn Any>>;
pub type gmObj = gmGenIndex<()>;
pub type gmID = u16;
pub type gmGen = u16; // There is no way you can even remotely get to 32kth generation -- Consider it a gift

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