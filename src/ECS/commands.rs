use super::*;
use world::gmWorld;

pub trait gmCommand: Any + Sized{
    fn CMD_ID() -> &'static str;
    fn execute(&mut self, IN_world: &mut gmWorld);
}