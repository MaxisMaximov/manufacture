use super::*;

use events::gmEvent;
use vars::*;
use resource::*;
use comp::*;
use builders::gmObjBuilder;
use storage::*;

pub struct gmWorld{
    pub gmObjs: gmObjStorage,
    pub components: gmWorld_COMPMAP,
    pub resources: gmWorld_RESMAP,
    pub events: gmWorld_EVENTMAP
}
impl gmWorld{

    pub fn new() -> Self{
        Self{
            gmObjs: gmObjStorage::new(),
            components: HashMap::new(),
            resources: HashMap::new(),
            events: gmWorld_EVENTMAP::new()
        }
    }

    pub fn fetch<'a, T>(&'a self) -> Fetch<'a, T> where T: gmComp + 'static{
        Fetch{
            inner: self.components.get(T::COMP_ID()).unwrap().as_ref().downcast_ref::<RefCell<T::COMP_STORAGE>>().unwrap().borrow()
        }
    }
    pub fn fetchMut<'a, T>(&'a self) -> FetchMut<'a, T> where T: gmComp + 'static{
        FetchMut{
            inner: self.components.get(T::COMP_ID()).unwrap().as_ref().downcast_ref::<RefCell<T::COMP_STORAGE>>().unwrap().borrow_mut()
        }
    }

    pub fn fetchRes<'a, T>(&'a self) -> FetchRes<'a, T> where T: gmRes + 'static{
        FetchRes{
            inner: self.resources.get(T::RES_ID()).unwrap().as_ref().downcast_ref::<RefCell<T>>().unwrap().borrow()
        }
    }
    pub fn fetchResMut<'a, T>(&'a self) -> FetchResMut<'a, T> where T: gmRes + 'static{
        FetchResMut{
            inner: self.resources.get(T::RES_ID()).unwrap().as_ref().downcast_ref::<RefCell<T>>().unwrap().borrow_mut()
        }
    }

    pub fn fetchEventReader<'a, T>(&'a self) -> EventReader<'a, T> where T: gmEvent + 'static{
        self.events.getEventReader()
    }

    pub fn fetchEventWriter<'a, T>(&'a self) -> EventWriter<'a, T> where T: gmEvent + 'static{
        self.events.getEventWriter()
    }

    pub fn registerComp<T>(&mut self) where T: gmComp + 'static{
        self.components.insert(
            T::COMP_ID(),
            Rc::new(RefCell::new(T::COMP_STORAGE::new()))
        );
    }
    pub fn unRegisterComp<T>(&mut self) where T: gmComp + 'static{
        self.components.remove(T::COMP_ID());
    }

    pub fn registerRes<T>(&mut self) where T: gmRes + 'static{
        self.resources.insert(
            T::RES_ID(), 
            Rc::new(RefCell::new(T::new()))
        );
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
        gmObjBuilder{
            gmObjID: self.gmObjs.insertNextFree(),
            worldRef: self,
        }
    }

    pub fn deleteGmObj(&mut self, IN_id: gmID){
        self.gmObjs.remove(IN_id);
            for COMP in self.components.values_mut(){
                COMP.clone().downcast::<RefCell<&mut dyn gmStorageDrop>>().unwrap().borrow_mut().drop(IN_id);
            }
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