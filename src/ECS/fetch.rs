use std::cell::{RefMut, Ref};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::*;

use comp::gmComp;
use events::gmEvent;

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

// So I'm thinking
// This thing is basically a wrapper, over a wrapper (Fetch/Mut), over another wrapper (Ref/Mut)
// And they all 3 implement Derefs, however Ref/Mut use it for safety stuff, Fetch/Mut and StorageRef only use it to give direct access to the storage/resource
// So in the end they both don't provide anything useful other than type clarity to what's a component and what's a resource fetch
// Also I'm not sure if Deref³ is a good idea for performance
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

pub type EventReader<'a, T: gmEvent> = Fetch<'a, Vec<T>>;
pub type EventWriter<'a, T: gmEvent> = FetchMut<'a, Vec<T>>;

pub type ReadStorage<'a, T: gmComp> = StorageRef<'a, T, Fetch<'a, T::COMP_STORAGE>>;
pub type WriteStorage<'a, T: gmComp> = StorageRef<'a, T, FetchMut<'a, T::COMP_STORAGE>>;