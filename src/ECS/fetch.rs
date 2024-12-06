use std::cell::{RefMut, Ref};
use std::ops::{Deref, DerefMut};

use super::*;

use comp::gmComp;
use events::gmEvent;
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


pub struct EventReader<'a, T: gmEvent>{
    pub inner: Ref<'a, Vec<T>>
}
impl<'a, T: gmEvent> Deref for EventReader<'a, T>{
    type Target = Ref<'a, Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct EventWriter<'a, T: gmEvent>{
    pub inner: RefMut<'a, Vec<T>>
}
impl<'a, T: gmEvent> Deref for EventWriter<'a, T>{
    type Target = RefMut<'a, Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T: gmEvent> DerefMut for EventWriter<'a, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}