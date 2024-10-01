use std::any::Any;
use std::collections::HashMap;

use super::*;

pub trait gmComp{
    fn COMP_ID() -> &'static str;
}

pub struct gmWorld{
    gmObjs: Vec<gmGenIndex<()>>,
    components: HashMap<&'static str, Box<dyn Any>>
}
impl gmWorld{
    pub fn fetch<T>(&self) -> &tests::vecStorage<T> where T: gmComp + 'static{
        self.components.get(T::COMP_ID()).unwrap().downcast_ref::<tests::vecStorage<T>>().unwrap()
    }
    pub fn fetchMut<T>(&mut self) -> &mut tests::vecStorage<T> where T: gmComp + 'static{
        self.components.get_mut(T::COMP_ID()).unwrap().downcast_mut::<tests::vecStorage<T>>().unwrap()
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

pub trait gmStorage: Any{
    type output;
    fn push(&mut self, IN_item: Self::output);
    fn pop(&mut self) -> Option<Self::output>;
}

pub struct gmGenIndex<T>{
    pub id: u16,
    pub gen: u16,
    pub val: T
}


mod tests{
    use super::*;

    pub fn main(){
        let mut world = gmWorld{
            components: HashMap::new(),
            gmObjs: Vec::new(),
        };

        world.gmObjs.push(gmGenIndex::<()>{id: 0, gen: 0, val: ()});
        world.components.insert(gmComp_Health::COMP_ID(), Box::new(vecStorage::<gmComp_Health>{inner: Vec::new()}));
        world.components.insert(gmComp_Pos::COMP_ID(), Box::new(vecStorage::<gmComp_Pos>{inner: Vec::new()}));

        world.fetchMut::<gmComp_Health>().push(gmComp_Health{val: 100});
        world.fetchMut::<gmComp_Pos>().push(gmComp_Pos{x: 0, y: 0});

        let mut dispatcher = gmDispatcher{systems: Vec::new()};

        dispatcher.systems.push(Box::new(gmSys_HP{}));

        dispatcher.dispatch(&mut world);

    }

    pub struct gmComp_Health{
        pub val: u16
    }
    impl gmComp for gmComp_Health{
        fn COMP_ID() -> &'static str {
            "gmComp_Health"
        }
    }

    pub struct gmComp_Pos{
        pub x: usize,
        pub y: usize
    }
    impl gmComp for gmComp_Pos{
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

    pub struct vecStorage<T>{
        inner: Vec<gmGenIndex<T>>
    }
    impl<T: 'static> gmStorage for vecStorage<T>{
        type output = T;

        fn push(&mut self, IN_item: Self::output) {
            self.inner.push(
                gmGenIndex{
                    id: self.inner.len() as u16,
                    gen: 0,
                    val: IN_item,
                }
            );
        }
    
        fn pop(&mut self) -> Option<Self::output> {
            if let Some(INDEX) = self.inner.pop(){
                return Some(INDEX.val);
            }
            None
        }
    }
}