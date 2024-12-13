use super::*;

use storage::*;

pub trait gmComp: Any + Sized{
    type COMP_STORAGE: gmStorage<Self> + Any;
    fn COMP_ID() -> &'static str;
}