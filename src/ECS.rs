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
    stages: Vec<gmDispatchStage>
}
impl gmDispatcher{
    pub fn new() -> Self{
        Self{
            stages: Vec::new()
        }
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
    pub systems: Vec<Box<dyn for<'a> gmSysRun<'a>>>
}
impl gmDispatchStage{
    pub fn new() -> Self{
        Self{
            systems: Vec::new()
        }
    }
    pub fn addSys<T>(mut self, IN_system: T) -> Self where T: for<'a> gmSystem<'a> + 'static{
        self.systems.push(Box::new(IN_system));
        self
    }
    pub fn dispatch(&mut self, IN_world: &mut gmWorld){
        for SYS in self.systems.iter_mut(){
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
        let w_nextIndex: gmID = if self.nextFree.len() == 0{
            // If there's no nextFree, length is always the next index
            self.gmObjMap.len() as gmID
        }else{
            // Else return first available in nextFree
            self.nextFree.pop_first().unwrap().0
        };

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

        let mut dispatcher = gmDispatcher::new();
        dispatcher.addStage(gmDispatchStage::new().addSys::<gmSys_input>(gmSys_input{}));
        dispatcher.addStage(gmDispatchStage::new().addSys::<gmSys_HP>(gmSys_HP{}));

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

        fn execute(&mut self, IN_data: Self::sysData) {
            for COMP_HP in IN_data.comp_HP.inner.iter_mut(){
                if COMP_HP.val.val <= 0{continue}
                COMP_HP.val.val -= 1
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

        fn execute(&mut self, IN_data: Self::sysData) {
            if !poll(Duration::from_secs(0)).unwrap(){
                IN_data.res_Input.res = KeyEvent{
                    code: KeyCode::Null,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    state: KeyEventState::NONE,
                };
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
                // Last index
                let idkfa_lastIndex = self.inner.len() - 1;
                // If it's the last element then just pop it
                if INDEX == idkfa_lastIndex{
                    return Some(self.inner.pop().unwrap().val)
                }
                // IF NOT
                // Update the ID in proxyMap
                *self.proxyMap.get_mut(&self.inner[idkfa_lastIndex].id).unwrap() = INDEX;
                // Swap the indexes
                self.inner.swap(INDEX, idkfa_lastIndex);
                // Pop the last one (now the one requested to remove)
                return Some(self.inner.pop().unwrap().val)
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