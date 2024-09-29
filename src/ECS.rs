use super::*;

pub trait gmComp{}

pub struct gmObj{
    ID: u16
}

pub struct gmWorld{
    gmObjs: Vec<gmObj>,
    hpVec: Vec<tests::gmComp_Health>,
    posVec: Vec<tests::gmComp_Pos>
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

mod tests{
    use super::*;

    pub fn main(){
        let mut world = gmWorld{
            gmObjs: Vec::new(),
            hpVec: Vec::new(),
            posVec: Vec::new(),
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

            for COMP_HP in IN_world.hpVec.iter_mut(){
                if COMP_HP.val <= 0{ continue}
                COMP_HP.val -= 1
            }
        }
    }
}