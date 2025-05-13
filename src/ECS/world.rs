use std::cell::{Ref, RefMut};

use super::*;

use events::gmEvent;
use vars::*;
use resource::*;
use comp::*;
use builders::gmObjBuilder;
use storage::*;
use fetch::*;
use entity::*;

pub struct gmWorld{
    gmObjs: gmObjStorage,
    components: gmWorld_COMPMAP,
    resources: gmWorld_RESMAP,
    events: gmWorld_EVENTMAP,
    commands: Rc<RefCell<gmWorld_CMDQUEUE>>
}
impl gmWorld{

    pub fn new() -> Self{
        Self{
            gmObjs: gmObjStorage::new(),
            components: gmWorld_COMPMAP::new(),
            resources: gmWorld_RESMAP::new(),
            events: gmWorld_EVENTMAP::new(),
            commands: Rc::new(RefCell::new(gmWorld_CMDQUEUE::new()))
        }
    }

    pub fn fetch<'a, T>(&'a self) -> ReadStorage<'a, T> where T: gmComp + 'static{
        ReadStorage::new(
            Fetch::new(
                Ref::map(self.components.get(T::COMP_ID()).expect(&format!("ERROR: Tried to fetch an unregistered component: {}", T::COMP_ID())).as_ref().borrow(), |idkfa| &idkfa.downcast::<T>().inner)
            )
        )
    }
    pub fn fetchMut<'a, T>(&'a self) -> WriteStorage<'a, T> where T: gmComp + 'static{
        WriteStorage::new(
            FetchMut::new(
                RefMut::map(self.components.get(T::COMP_ID()).expect(&format!("ERROR: Tried to fetch an unregistered component: {}", T::COMP_ID())).as_ref().borrow_mut(), |idkfa| &mut idkfa.downcast_mut::<T>().inner)
            )
        )
    }

    pub fn fetchRes<'a, T>(&'a self) -> Fetch<'a, T> where T: gmRes + 'static{
        Fetch::new(
            Ref::map(self.resources.get(T::RES_ID()).expect(&format!("ERROR: Tried to fetch an unregistered resource: {}", T::RES_ID())).as_ref().borrow(), |idkfa| idkfa.downcast_ref::<T>().unwrap())
        )
    }
    pub fn fetchResMut<'a, T>(&'a self) -> FetchMut<'a, T> where T: gmRes + 'static{
        FetchMut::new(
            RefMut::map(self.resources.get(T::RES_ID()).expect(&format!("ERROR: Tried to fetch an unregistered resource: {}", T::RES_ID())).as_ref().borrow_mut(), |idkfa| idkfa.downcast_mut::<T>().unwrap())
        )
    }

    pub fn fetchEventReader<'a, T>(&'a self) -> EventReader<'a, T> where T: gmEvent + 'static{
        self.events.getEventReader()
    }

    pub fn fetchEventWriter<'a, T>(&'a self) -> EventWriter<'a, T> where T: gmEvent + 'static{
        self.events.getEventWriter()
    }

    pub fn registerComp<T>(&mut self) where T: gmComp + 'static{
        use std::collections::hash_map::Entry;
        match self.components.entry(T::COMP_ID()){
            Entry::Occupied(_) => panic!("ERROR: Attempted to override an existing component: {}", T::COMP_ID()),
            Entry::Vacant(ENTRY) => ENTRY.insert(
                    Rc::new(
                        RefCell::new(
                            gmStorageContainer::<T>{inner: T::COMP_STORAGE::new()})))
        };
    }
    pub fn unRegisterComp<T>(&mut self) where T: gmComp + 'static{
        self.components.remove(T::COMP_ID());
    }

    pub fn registerRes<T>(&mut self) where T: gmRes + 'static{
        use std::collections::hash_map::Entry;
        match self.resources.entry(T::RES_ID()){
            Entry::Occupied(_) => panic!("ERROR: Attempted to override an existing resource: {}", T::RES_ID()),
            Entry::Vacant(ENTRY) => ENTRY.insert(Rc::new(RefCell::new(T::new())))
        };
    }
    pub fn unRegisterRes<T>(&mut self) where T: gmRes + 'static{
        self.resources.remove(T::RES_ID());
    }

    pub fn registerEvent<T>(&mut self) where T: gmEvent + 'static{
        self.events.registerEvent::<T>();
    }
    pub fn unRegisterEvent<T>(&mut self) where T: gmEvent + 'static{
        self.events.unRegisterEvent::<T>();
    }

    pub fn createGmObj(&mut self) -> gmObjBuilder{
        gmObjBuilder::new(
             self.gmObjs.insertNextFree(),
            self,
        )
    }
    pub fn deleteGmObj(&mut self, IN_id: gmID) -> Result<(), ()>{
        match self.gmObjs.remove(IN_id){
            Ok(_) => {
                for COMP in self.components.values_mut(){
                    COMP.as_ref().borrow_mut().drop(&IN_id);
                }
                return Ok(())
            }
            Err(_) => {Err(())}
        }
    }
    
    pub fn fetchCommandWriter<'a>(&'a self) -> FetchMut<'a, gmWorld_CMDQUEUE>{
        FetchMut::new(
            RefMut::map(self.commands.as_ref().borrow_mut(), |idkfa| idkfa),
        )
    }

    pub fn commandsExec(&mut self){
        loop{
            let idkfa = self.commands.as_ref().borrow_mut().pop();
            match idkfa{
                Some(COMMAND) => COMMAND.execute(self),
                None => break,
            }
        }
    }

    pub fn endTick(&mut self){
        self.commandsExec();
        self.events.switchNClear();
    }
}

pub struct gmObjStorage{
    pub gmObjMap: HashMap<gmID, Entity>,
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
        self.gmObjMap.insert(IN_id, Entity::new(IN_id));
    }

    pub fn insertNextFree(&mut self) -> gmID{
        let w_nextIndex: gmID = self.nextFree.pop_first().unwrap_or((self.gmObjMap.len() as gmID, ())).0;

        self.insert(w_nextIndex);

        return w_nextIndex
    }

    pub fn remove(&mut self, IN_id: gmID) -> Result<(), ()>{
        if self.gmObjMap.remove(&IN_id).is_some(){
            self.nextFree.insert(IN_id, ());
            return Ok(());
        }
        Err(())
    }
}