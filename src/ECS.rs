use std::any::Any;
use std::collections::HashMap;

use super::*;

pub trait gmComp: Any + Sized{
    type COMP_STORAGE: gmStorage<Self>;
    fn COMP_ID() -> &'static str;
}

pub trait gmRes: Any{
    fn RES_ID() -> &'static str;
}

pub struct gmWorld{
    pub gmObjs: Vec<gmGenIndex<()>>,
    pub components: HashMap<&'static str, Box<dyn Any>>,
    pub resources: HashMap<&'static str, Box<dyn Any>>,
}
impl gmWorld{
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
            Box::new(tests::vecStorage::<T>{inner: Vec::new()})
        );
    }
    pub fn unRegisterComp<T>(&mut self) where T: gmComp + 'static{
        self.components.remove(T::COMP_ID());
    }

    pub fn registerRes<T>(&mut self, IN_res: T) where T: gmRes + 'static{
        self.resources.insert(T::RES_ID(), Box::new(IN_res));
    }
    pub fn unRegisterRes<T>(&mut self) where T: gmRes + 'static{
        self.resources.remove(T::RES_ID());
    }

    pub fn createGmObj(&mut self) -> u16{
        let idkfa_ID = self.gmObjs.len() as u16;
        self.gmObjs.push(gmGenIndex{
            id: idkfa_ID,
            gen: 0,
            val: (),
        });
        idkfa_ID
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
    fn push(&mut self, IN_item: T);
    fn pop(&mut self) -> Option<T>;
}

pub struct gmGenIndex<T>{
    pub id: u16,
    pub gen: u16,
    pub val: T
}


mod tests{
    use event::*;
    use time::Duration;

    use super::*;

    pub fn main(){
        let mut world = gmWorld{
            gmObjs: Vec::new(),
            components: HashMap::new(),
            resources: HashMap::new(),
        };

        world.createGmObj();
        world.registerComp::<gmComp_Health>();
        world.registerComp::<gmComp_Pos>();

        world.registerRes::<gmRes_deltaT>(gmRes_deltaT{res: Duration::from_secs(0)});
        world.registerRes::<gmRes_PInput>(gmRes_PInput{res: KeyEvent{code: event::KeyCode::Null, kind: event::KeyEventKind::Release, modifiers: KeyModifiers::NONE, state: KeyEventState::NONE}});

        world.fetchMut::<gmComp_Health>().push(gmComp_Health{val: 100});
        world.fetchMut::<gmComp_Pos>().push(gmComp_Pos{x: 0, y: 0});

        let mut dispatcher = gmDispatcher{systems: Vec::new()};

        dispatcher.systems.push(Box::new(gmSys_input{}));
        dispatcher.systems.push(Box::new(gmSys_HP{}));

        dispatcher.dispatch(&mut world);

    }

    pub struct gmComp_Health{
        pub val: u16
    }
    impl gmComp for gmComp_Health{
        type COMP_STORAGE = vecStorage<Self>;
        fn COMP_ID() -> &'static str {
            "gmComp_Health"
        }
    }

    pub struct gmComp_Pos{
        pub x: usize,
        pub y: usize
    }
    impl gmComp for gmComp_Pos{
        type COMP_STORAGE = vecStorage<Self>;
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

    pub struct vecStorage<T>{
        pub inner: Vec<gmGenIndex<T>>
    }
    impl<T: 'static> gmStorage<T> for vecStorage<T>{

        fn push(&mut self, IN_item: T) {
            self.inner.push(
                gmGenIndex{
                    id: self.inner.len() as u16,
                    gen: 0,
                    val: IN_item,
                }
            );
        }
    
        fn pop(&mut self) -> Option<T> {
            if let Some(INDEX) = self.inner.pop(){
                return Some(INDEX.val);
            }
            None
        }
    }

    pub struct gmRes_deltaT{
        res: Duration
    }
    impl gmRes for gmRes_deltaT{
        fn RES_ID() -> &'static str {
            "gmRes_deltaT"
        }
    }

    pub struct gmRes_PInput{
        res: KeyEvent
    }
    impl gmRes for gmRes_PInput{
        fn RES_ID() -> &'static str {
            "gmResPInput"
        }
    }
}