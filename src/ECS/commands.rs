use std::marker::PhantomData;

use super::*;
use comp::gmComp;
use storage::gmStorage;
use vars::gmID;
use world::gmWorld;

pub trait gmCommand: Any{
    fn CMD_ID() -> &'static str;
    fn execute(&mut self, IN_world: &mut gmWorld);
}

pub struct cmd_SpawnGmObj{}
impl gmCommand for cmd_SpawnGmObj{
    fn CMD_ID() -> &'static str {
        "cmd_SpawnGmObj"
    }

    fn execute(&mut self, IN_world: &mut gmWorld) {
        IN_world.createGmObj();
    }
}

pub struct cmd_addComp<T: gmComp>{
    pub gmObj: gmID,
    pub comp: T
}
impl<T: gmComp> gmCommand for cmd_addComp<T>{
    fn CMD_ID() -> &'static str {
        "cmd_addComp"
    }

    fn execute(&mut self, IN_world: &mut gmWorld) {
        IN_world.fetchMut::<T>().insert(self.gmObj, self.comp);
    }
}

pub struct cmd_removeComp<T: gmComp>{
    pub gmObj: gmID,
    pub _phantom: PhantomData<T>
}
impl<T: gmComp> gmCommand for cmd_removeComp<T>{
    fn CMD_ID() -> &'static str {
        "cmd"
    }

    fn execute(&mut self, IN_world: &mut gmWorld) {
        IN_world.fetchMut::<T>().remove(self.gmObj);
    }
}