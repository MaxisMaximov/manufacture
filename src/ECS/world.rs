use super::*;

use vars::*;
use resource::*;
use comp::*;
use builders::gmObjBuilder;
use storage::*;

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

    pub fn fetch<T>(&self) -> Fetch<T::COMP_STORAGE> where T: gmComp + 'static{
        Fetch{
            inner: self.components.get(T::COMP_ID()).unwrap().clone().downcast::<RefCell<T::COMP_STORAGE>>().unwrap()
        }
    }
    pub fn fetchMut<T>(&mut self) -> FetchMut<T::COMP_STORAGE> where T: gmComp + 'static{
        FetchMut{
            inner: self.components.get_mut(T::COMP_ID()).unwrap().clone().downcast::<RefCell<T::COMP_STORAGE>>().unwrap()
        }
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
            Box::new(Rc::new(RefCell::new(T::COMP_STORAGE::new())))
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