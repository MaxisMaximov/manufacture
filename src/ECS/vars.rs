use std::cell::{RefMut, Ref};

use super::*;

use misc::gmGenIndex;
use comp::gmComp;
use resource::gmRes;

pub struct Fetch<'a, T: gmComp>{
    pub inner: Ref<'a, T::COMP_STORAGE>
}
pub struct FetchMut<'a, T: gmComp>{
    pub inner: RefMut<'a, T::COMP_STORAGE>
}

pub struct FetchRes<'a, T: gmRes>{
    pub inner: Ref<'a, T>
}
pub struct FetchResMut<'a, T: gmRes>{
    pub inner: RefMut<'a, T>
}

pub type gmWorld_COMPMAP = HashMap<&'static str, Box<Rc<dyn Any>>>;
pub type gmWorld_RESMAP = HashMap<&'static str, Box<Rc<dyn Any>>>;
pub type gmObj = gmGenIndex<()>;
pub type gmID = u16;
pub type gmGen = u16; // There is no way you can even remotely get to 32kth generation -- Consider it a gift