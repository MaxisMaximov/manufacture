use super::*;

pub trait gmComp{}

pub struct gmObj{
    ID: u16
}

pub struct gmWorld{
    gmObjs: Vec<gmObj>,
    hpVec: tests::hpStorage,
    posVec: tests::posStorage
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

pub trait gmStorage<T>{
    fn push(&mut self, IN_item: T);
    fn pop(&mut self) -> Option<T>;
}

mod tests{
    use super::*;

    pub fn main(){
        let mut world = gmWorld{
            gmObjs: Vec::new(),
            hpVec: hpStorage{inner: Vec::new()},
            posVec: posStorage{inner: Vec::new()},
        };

        world.gmObjs.push(gmObj{ID: 0});
        world.hpVec.push(gmComp_Health{val: 100});
        world.posVec.push(gmComp_Pos{x: 0, y: 0});

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

            for COMP_HP in IN_world.hpVec.inner.iter_mut(){
                if COMP_HP.val <= 0{continue}
                COMP_HP.val -= 1
            }
        }
    }

    pub struct hpStorage{
        pub inner: Vec<gmComp_Health>
    }
    impl gmStorage<gmComp_Health> for hpStorage{
        fn push(&mut self, IN_item: gmComp_Health) {
            self.inner.push(IN_item);
        }
    
        fn pop(&mut self) -> Option<gmComp_Health> {
            self.inner.pop()
        }
    }

    pub struct posStorage{
        pub inner: Vec<gmComp_Pos>
    }
    impl gmStorage<gmComp_Pos> for posStorage{
        fn push(&mut self, IN_item: gmComp_Pos) {
            self.inner.push(IN_item);
        }
    
        fn pop(&mut self) -> Option<gmComp_Pos> {
            self.inner.pop()
        }
    }
}