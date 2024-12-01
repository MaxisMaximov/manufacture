use super::*;

use storage::*;

pub trait gmComp: Any + Sized{
    type COMP_STORAGE: gmStorage<Self>;
    fn COMP_ID() -> &'static str;
}