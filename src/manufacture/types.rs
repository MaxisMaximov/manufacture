use std::cell::{Ref, RefMut};
use std::ops::{Index, IndexMut};
use std::ops::{Deref, DerefMut};

use crossterm::style::Color;

use super::resources::{gmEvent, res_Events};

pub type Vector2 = (isize, isize);

#[derive(Clone, Copy)]
pub struct StyleSet{
    pub ch: char,
    pub fg: Color,
    pub bg: Color
}
impl Default for StyleSet{
    fn default() -> Self {
        Self { ch: ' ', fg: Color::White, bg: Color::Black }
    }
}

pub struct FetchEvent<'a, T: gmEvent>{
    pub inner: Ref<'a, Vec<T>>
}
impl<'a, T: gmEvent> Deref for FetchEvent<'a, T>{
    type Target = Ref<'a, Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct FetchEventMut<'a, T: gmEvent>{
    pub inner: RefMut<'a, Vec<T>>
}
impl<'a, T: gmEvent> Deref for FetchEventMut<'a, T>{
    type Target = RefMut<'a, Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T: gmEvent> DerefMut for FetchEventMut<'a, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct DoubleDArray<T, const X: usize, const Y: usize>{
    pub inner: [[T; X]; Y], // RUST PLEASE LET ME USE CONST EXPRESSIONS WITH GENERICS
    dummyT: T
}
impl<T: Default + Copy, const X: usize, const Y: usize> DoubleDArray<T, X, Y>{
    pub fn new() -> Self{
        Self{
            inner: [[T::default(); X]; Y],
            dummyT: T::default()
        }
    }
}
impl<T, const X: usize, const Y: usize> Index<Vector2> for DoubleDArray<T, X, Y>{
    type Output = T;

    fn index(&self, index: Vector2) -> &Self::Output {
        if let Some(ROW) = self.inner.get((index.1 as usize).wrapping_add(Y/2)){
            if let Some(TYPE) = ROW.get((index.0 as usize).wrapping_add(X/2)){
                return TYPE
            }
        }
        &self.dummyT
    }
}
impl<T, const X: usize, const Y: usize> IndexMut<Vector2> for DoubleDArray<T, X, Y>{
    fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
        if let Some(ROW) = self.inner.get_mut((index.1 as usize).wrapping_add(Y/2)){
            if let Some(TYPE) = ROW.get_mut((index.0 as usize).wrapping_add(X/2)){
                return TYPE
            }
        }
        &mut self.dummyT
    }
}