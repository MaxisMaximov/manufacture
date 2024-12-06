use std::cell::{RefMut, Ref};
use std::marker::PhantomData;
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
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub struct StorageRef<'a, T: gmComp, D>{
    pub data: D,
    pub _phantom: PhantomData<&'a T>
}
impl<'a, T: gmComp, D> Deref for StorageRef<'a, T, D>{
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, T: gmComp, D> DerefMut for StorageRef<'a, T, D>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub type readStorage<'a, T: gmComp> = StorageRef<'a, T, Fetch<'a, T::COMP_STORAGE>>;
pub type writeStorage<'a, T: gmComp> = StorageRef<'a, T, FetchMut<'a, T::COMP_STORAGE>>;