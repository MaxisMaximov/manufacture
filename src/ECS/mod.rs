#![allow(dead_code)]
use std::any::Any;
use std::collections::{BTreeMap, HashMap};

use super::*;

mod comp;
mod storage;
mod system;
mod vars;
mod world;
mod resource;
mod builders;
mod dispatcher;
mod misc;

use comp::*;
use storage::*;
use system::*;
use vars::*;
use world::*;
use resource::*;
use builders::*;
use dispatcher::*;
use misc::*;

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

        let mut dispatcher = gmDispatcher::new()
            .withSys::<gmSys_input>(&[])
            .withSys::<gmSys_HP>(&[]);

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

        fn new() -> Self {
            Self{}
        }

        fn SYS_ID() -> &'static str {
            "gmSys_HP"
        }

        fn execute(&mut self, IN_data: Self::sysData) {
            for COMP_HP in IN_data.comp_HP.inner.iter_mut(){
                if COMP_HP.val.val > 0{
                    COMP_HP.val.val -= 1
                }
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

        fn new() -> Self {
            Self{}
        }

        fn SYS_ID() -> &'static str {
            "gmSys_input"
        }

        fn execute(&mut self, IN_data: Self::sysData) {
            if !poll(Duration::from_secs(0)).unwrap(){
                IN_data.res_Input.res = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE);
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