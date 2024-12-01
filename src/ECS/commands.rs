use super::*;
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