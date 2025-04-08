use super::Vector2;

use std::ops::{Index, IndexMut};

pub struct CartesianGrid<T>{ // It's Cartesian and it's a grid -- Just to not confuse with *proper* 2D arrays
    pub inner: Vec<T>,
    size: (usize, usize),
    dummy: T
}
impl<T: Default + Copy> CartesianGrid<T>{
    pub fn new(SizeX: usize, SizeY: usize) -> Self{
        Self{
            inner: vec![T::default(); SizeX * SizeY],
            size: (SizeX, SizeY),
            dummy: T::default()
        }
    }
}
impl<T> Index<Vector2> for CartesianGrid<T>{
    type Output = T;

    fn index(&self, index: Vector2) -> &Self::Output {

        let xpos = (index.0 as usize).wrapping_add(self.size.0/2);
        let ypos = (index.1 as usize).wrapping_add(self.size.1/2);

        if xpos >= self.size.0 || ypos >= self.size.1{
            return &self.dummy
        }

        if let Some(CELL) = self.inner.get(xpos + ypos * self.size.0){
            return CELL
        }
        &self.dummy
    }
}
impl<T> IndexMut<Vector2> for CartesianGrid<T>{
    fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
        
        let xpos = (index.0 as usize).wrapping_add(self.size.0/2);
        let ypos = (index.1 as usize).wrapping_add(self.size.1/2);

        if xpos >= self.size.0 || ypos >= self.size.1{
            return &mut self.dummy
        }

        if let Some(CELL) = self.inner.get_mut(xpos + ypos * self.size.0){
            return CELL
        }
        &mut self.dummy
    }
}