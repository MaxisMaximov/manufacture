use std::cell::{Ref, RefMut};
use std::ops::{Index, IndexMut};

use crossterm::style::Color;

use super::resources::gmEvent;

// World size in chunks
pub const WORLD_X_MIN: isize = -5;
pub const WORLD_X_MAX: isize = 5;
pub const WORLD_Y_MIN: isize = -5;
pub const WORLD_Y_MAX: isize = 5;

pub const CHUNK_X: isize = 8;
pub const CHUNK_Y: isize = 8;

pub const RENDER_VIEWPORT_X_MIN: isize = -5;
pub const RENDER_VIEWPORT_X_MAX: isize = 5;
pub const RENDER_VIEWPORT_Y_MIN: isize = -5;
pub const RENDER_VIEWPORT_Y_MAX: isize = 5;

// Don't touch, full size of the Viewport render
pub const RENDER_VIEWPORT_X: usize = (RENDER_VIEWPORT_X_MAX - RENDER_VIEWPORT_X_MIN) as usize;
pub const RENDER_VIEWPORT_Y: usize = (RENDER_VIEWPORT_Y_MAX - RENDER_VIEWPORT_Y_MIN) as usize;

// Yeah smaller than the old buffer, realized I don't use most of it anyway lol
// Yet
pub const RENDER_BUFFER_X: usize = 20;
pub const RENDER_BUFFER_Y: usize = 20;

// To still render objects if they're "technically" in view
pub const RENDER_MARGIN: isize = 4;

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

pub struct FetchEventMut<'a, T: gmEvent>{
    pub inner: RefMut<'a, Vec<T>>
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