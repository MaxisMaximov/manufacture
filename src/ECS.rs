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
            gmObjs: gmObjStorage{
                gmObjMap: HashMap::new(),
                nextFree: BTreeMap::new(),
            },
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
        let w_nextIndex: gmID = if self.gmObjs.nextFree.len() == 0{
                self.gmObjs.gmObjMap.len() as gmID
            }else{
                self.gmObjs.nextFree.pop_first().unwrap().0
            };
        self.gmObjs.gmObjMap.insert(w_nextIndex, gmObj{
            id: w_nextIndex,
            gen: 0,
            val: (),
        });
        gmObjBuilder{
            gmObj: &self.gmObjs.gmObjMap.get(&w_nextIndex).unwrap(),
            CompMapRef: &mut self.components,
        }
    }
}

pub trait gmSystem{
    fn execute(&mut self, IN_world: &mut gmWorld);
}

pub struct gmDispatcher{
    systems: Vec<Box<dyn gmSystem>>
}
impl gmDispatcher{
    pub fn dispatch(&mut self, IN_world: &mut gmWorld){
        for SYS in self.systems.iter_mut(){
            SYS.execute(IN_world);
        }
    }
}

pub trait gmStorage<T>: Any{
    fn new() -> Self;
    fn insert(&mut self, IN_id: gmID, IN_item: T);
    fn remove(&mut self, IN_id: gmID) -> Option<T>;
}

#[derive(Clone, Copy)]
pub struct gmGenIndex<T>{
    pub id: gmID,
    pub gen: gmGen,
    pub val: T
}

pub struct gmObjStorage{
    pub gmObjMap: HashMap<gmID, gmObj>,
    pub nextFree: BTreeMap<gmID, ()>,
}

pub struct gmObjBuilder<'a>{
    pub gmObj: &'a gmObj,
    pub CompMapRef: &'a mut gmWorld_COMPMAP
}
impl gmObjBuilder<'_>{
    pub fn addComp<T>(self, IN_comp: T) -> Self where T:gmComp{
        // I gotta deal with this
        self.CompMapRef.get_mut(T::COMP_ID()).unwrap().downcast_mut::<T::COMP_STORAGE>().unwrap().insert(self.gmObj.id, IN_comp);
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

        let mut dispatcher = gmDispatcher{systems: Vec::new()};

        dispatcher.systems.push(Box::new(gmSys_input{}));
        dispatcher.systems.push(Box::new(gmSys_HP{}));

        dispatcher.dispatch(&mut world);
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
    impl gmSystem for gmSys_HP{
        fn execute(&mut self, IN_world: &mut gmWorld) {

            for COMP_HP in IN_world.fetchMut::<gmComp_Health>().inner.iter_mut(){
                if COMP_HP.val.val <= 0{continue}
                COMP_HP.val.val -= 1
            }
        }
    }

    pub struct gmSys_input{}
    impl gmSystem for gmSys_input{
        fn execute(&mut self, IN_world: &mut gmWorld) {
            let mut INPUT_LOCK = IN_world.fetchResMut::<gmRes_PInput>();
            if !poll(Duration::from_secs(0)).unwrap(){
                INPUT_LOCK.res = KeyEvent{
                    code: KeyCode::Null,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    state: KeyEventState::NONE,
                };
                return
            }
            if let Event::Key(KEY) = read().unwrap(){
                INPUT_LOCK.res = KEY;
                return
            }
        }
    }

    pub struct denseVecStorage<T>{
        pub proxyMap: HashMap<gmID, usize>,
        pub inner: Vec<gmGenIndex<T>>
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
                gmGenIndex{
                    id: IN_id,
                    gen: 0,
                    val: IN_item,
                }
            );
        }
    
        fn remove(&mut self, IN_id: gmID) -> Option<T> {
            
            if let Some(INDEX) = self.proxyMap.remove(&IN_id){
                // Last index
                let idkfa_index = self.inner.len() - 1;
                // If it's the last element then just pop it
                if INDEX == idkfa_index{
                    return Some(self.inner.pop().unwrap().val)
                }
                // IF NOT
                // Update the ID in proxyMap
                *self.proxyMap.get_mut(&self.inner[idkfa_index].id).unwrap() = INDEX;
                // Swap the indexes
                self.inner.swap(INDEX, idkfa_index);
                // Pop the last one (now the one requested to remove)
                return Some(self.inner.pop().unwrap().val)
            }
            None

        }
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