use std::cell::{RefMut, Ref};
use std::ops::{Deref, DerefMut};

use super::*;

use comp::gmComp;
use events::gmEvent;
use resource::gmRes;

pub struct Fetch<'a, T>{
    pub data: Ref<'a, T>,
}
impl<'a, T> Deref for Fetch<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
pub struct FetchMut<'a, T>{
    pub data: RefMut<'a, T>
}
impl<'a, T> Deref for FetchMut<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, T> DerefMut for FetchMut<'a, T>{
    fn deref_mut(&mut self) -> &Self::Target {
        &mut self.data
    }
}

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