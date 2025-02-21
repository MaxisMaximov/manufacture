use super::Vector2;

use std::ops::{Index, IndexMut};

pub struct CartesianGrid<T>{ // It's Cartesian and it's a grid -- Just to not confuse with *proper* 2D arrays
    pub inner: Vec<T>,
    size: (usize, usize),
    dummyT: T
}
impl<T: Default + Copy> CartesianGrid<T>{
    pub fn new(IN_x: usize, IN_y: usize) -> Self{
        Self{
            inner: vec![T::default(); IN_x * IN_y],
            size: (IN_x, IN_y),
            dummyT: T::default()
        }
    }
}
impl<T> Index<Vector2> for CartesianGrid<T>{
    type Output = T;

    fn index(&self, index: Vector2) -> &Self::Output {

        let X = (index.0 as usize).wrapping_add(self.size.0/2);
        let Y = (index.1 as usize).wrapping_add(self.size.1/2);

        if X >= self.size.0 || Y >= self.size.1{
            return &self.dummyT
        }

        if let Some(CELL) = self.inner.get(X + Y * self.size.0){
            return CELL
        }
        &self.dummyT
    }
}
impl<T> IndexMut<Vector2> for CartesianGrid<T>{
    fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
        
        let X = (index.0 as usize).wrapping_add(self.size.0/2);
        let Y = (index.1 as usize).wrapping_add(self.size.1/2);

        if X >= self.size.0 || Y >= self.size.1{
            return &mut self.dummyT
        }

        if let Some(CELL) = self.inner.get_mut(X + Y * self.size.0){
            return CELL
        }
        &mut self.dummyT
    }
}