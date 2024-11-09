use super::*;

use misc::gmGenIndex;
use comp::gmComp;
use resource::gmRes;

pub struct Fetch<T: gmComp>{
    pub inner: Rc<RefCell<T::COMP_STORAGE>>
}
pub struct FetchMut<T: gmComp>{
    pub inner: Rc<RefCell<T::COMP_STORAGE>>
}

pub struct FetchRes<T: gmRes>{
    pub inner: Rc<RefCell<T>>
}
pub struct FetchResMut<T: gmRes>{
    pub inner: Rc<RefCell<T>>
}

pub type gmWorld_COMPMAP = HashMap<&'static str, Box<Rc<dyn Any>>>;
pub type gmWorld_RESMAP = HashMap<&'static str, Box<Rc<dyn Any>>>;
pub type gmObj = gmGenIndex<()>;
pub type gmID = u16;
pub type gmGen = u16; // There is no way you can even remotely get to 32kth generation -- Consider it a gift