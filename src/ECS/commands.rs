use std::marker::PhantomData;

use super::*;
use comp::gmComp;
use prefab::gmPrefab;
use storage::gmStorage;
use vars::gmID;
use world::gmWorld;

pub trait gmCommand: Any{
    fn CMD_ID(&self) -> &'static str;
    fn execute(&self, IN_world: &mut gmWorld);
}

pub struct cmd_SpawnGmObj{}
impl gmCommand for cmd_SpawnGmObj{
    fn CMD_ID(&self) -> &'static str {
        "cmd_SpawnGmObj"
    }

    fn execute(&self, IN_world: &mut gmWorld) {
        IN_world.createGmObj();
    }
}

pub struct cmd_addComp<T: gmComp>{
    pub gmObj: gmID,
    pub comp: T
}
impl<T: gmComp + Clone> gmCommand for cmd_addComp<T>{
    fn CMD_ID(&self) -> &'static str {
        "cmd_addComp"
    }

    fn execute(&self, IN_world: &mut gmWorld) {
        IN_world.fetchMut::<T>().insert(self.gmObj.clone(), self.comp.to_owned());
    }
}

pub struct cmd_removeComp<T: gmComp>{
    pub gmObj: gmID,
    pub _phantom: PhantomData<T>
}
impl<T: gmComp> gmCommand for cmd_removeComp<T>{
    fn CMD_ID(&self) -> &'static str {
        "cmd"
    }

    fn execute(&self, IN_world: &mut gmWorld) {
        IN_world.fetchMut::<T>().remove(&self.gmObj);
    }
}

pub struct cmd_spawnPrefab<T: gmPrefab>{
    pub prefab: T
}
impl<T: gmPrefab> gmCommand for cmd_spawnPrefab<T>{
    fn CMD_ID(&self) -> &'static str {
        "cmd_SpawnPrefab"
    }

    fn execute(&self, IN_world: &mut gmWorld) {
        self.prefab.spawn(IN_world.createGmObj());
    }
}