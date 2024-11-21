use std::cell::{Ref, RefMut};

use super::resources::gmEvent;

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

// To still render objects if they're "technically" in view
pub const RENDER_MARGIN: isize = 4;

pub type Vector2 = (isize, isize);

pub struct FetchEvent<'a, T: gmEvent>{
    pub inner: Ref<'a, Vec<T>>
}

pub struct FetchEventMut<'a, T: gmEvent>{
    pub inner: RefMut<'a, Vec<T>>
}

pub struct DoubleDArray<T, const X: usize, const Y: usize>{
    inner: [[T; Y]; X] // RUST PLEASE LET ME USE CONST EXPRESSIONS WITH GENERICS
}