use std::any::Any;

use super::*;

pub trait gmComp{}

pub struct gmObj{
    ID: u16
}

pub struct gmWorld{
    gmObjs: Vec<gmObj>,
    components: Vec<Box<dyn Any>>,
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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn main(){
        let mut world = gmWorld{
            components: Vec::new(),
            gmObjs: Vec::new(),
        };

        world.gmObjs.push(gmObj{ID: 0});
        world.components.push(Box::new(hpStorage{inner: Vec::new()}));
        world.components.push(Box::new(posStorage{inner: Vec::new()}));

        world.components[0].downcast_mut::<hpStorage>().unwrap().push(gmComp_Health{val: 100});
        world.components[1].downcast_mut::<posStorage>().unwrap().push(gmComp_Pos{x: 0, y: 0});

        let mut dispatcher = gmDispatcher{systems: Vec::new()};

        dispatcher.systems.push(Box::new(gmSys_HP{}));

        dispatcher.dispatch(&mut world);

    }

    pub struct gmComp_Health{
        pub val: u16
    }
    impl gmComp for gmComp_Health{}

    pub struct gmComp_Pos{
        pub x: usize,
        pub y: usize
    }
    impl gmComp for gmComp_Pos{}

    pub struct gmSys_HP{}
    impl gmSystem for gmSys_HP{
        fn execute(&mut self, IN_world: &mut gmWorld) {

            for COMP_HP in IN_world.components[0].downcast_mut::<hpStorage>().unwrap().inner.iter_mut(){
                if COMP_HP.val.val <= 0{continue}
                COMP_HP.val.val -= 1
            }
        }
    }

    pub struct hpStorage{
        pub inner: Vec<gmGenIndex<gmComp_Health>>
    }
    impl gmStorage for hpStorage{

        type output = gmComp_Health;

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

    pub struct posStorage{
        pub inner: Vec<gmGenIndex<gmComp_Pos>>
    }
    impl gmStorage for posStorage{

        type output = gmComp_Pos;

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
                return Some(INDEX.val)
            }
            None
        }
    }
}