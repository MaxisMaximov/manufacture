use std::cell::{RefMut, Ref};
use std::ops::{Deref, DerefMut};

use super::*;

use misc::gmGenIndex;
use comp::gmComp;
use resource::gmRes;

pub struct Fetch<'a, T: gmComp>{
    pub inner: Ref<'a, T::COMP_STORAGE>
}
impl<'a, T: gmComp> Deref for Fetch<'a, T>{
    type Target = Ref<'a, T::COMP_STORAGE>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
pub struct FetchMut<'a, T: gmComp>{
    pub inner: RefMut<'a, T::COMP_STORAGE>
}
impl<'a, T: gmComp> Deref for FetchMut<'a, T>{
    type Target = RefMut<'a, T::COMP_STORAGE>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T: gmComp> DerefMut for FetchMut<'a, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct FetchRes<'a, T: gmRes>{
    pub inner: Ref<'a, T>
}
impl<'a, T: gmRes> Deref for FetchRes<'a, T>{
    type Target = Ref<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
pub struct FetchResMut<'a, T: gmRes>{
    pub inner: RefMut<'a, T>
}
impl<'a, T: gmRes> Deref for FetchResMut<'a, T>{
    type Target = RefMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T: gmRes> DerefMut for FetchResMut<'a, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub type gmWorld_COMPMAP = HashMap<&'static str, Rc<dyn Any>>;
pub type gmWorld_RESMAP = HashMap<&'static str, Rc<dyn Any>>;
pub type gmObj = gmGenIndex<()>;
pub type gmID = u16;
pub type gmGen = u16; // There is no way you can even remotely get to 32kth generation -- Consider it a gift