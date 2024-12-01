use super::*;

use storage::*;

pub trait gmComp: Any + Sized + Clone + Copy{
    type COMP_STORAGE: gmStorage<Self>;
    fn COMP_ID() -> &'static str;
}