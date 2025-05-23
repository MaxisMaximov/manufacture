use types::Vector2;
use constants::*;

use super::*;

pub fn util_coordConvert(IN_coords: Vector2) -> (Vector2, Vector2){
    let mut w_chunkCoords: Vector2 = (IN_coords.0 / CHUNK_X, IN_coords.1 / CHUNK_Y);
    let w_tileCoords: Vector2 = (IN_coords.0 % CHUNK_X, IN_coords.1 % CHUNK_Y);

    // Skip over a chunk/s to not end up in (0, 0)
    if IN_coords.0 < 0 && w_tileCoords.0 != 0{
        w_chunkCoords.0 -= 1;
    }
    if IN_coords.1 < 0 && w_tileCoords.1 != 0{
        w_chunkCoords.1 -= 1;
    }

    (w_chunkCoords, w_tileCoords)
}

pub mod rangeStuff{
    use std::ops::{Add, Range};

    pub trait RangeOffset<U>{
        fn offset(self, IN_offset: U) -> Self;
    }
    impl<T, U> RangeOffset<U> for Range<T> where T: Add<U, Output = T>, U: Copy{
        fn offset(self, IN_offset: U) -> Self {
            Range{
                start: self.start + IN_offset,
                end: self.end + IN_offset,
            }
        }
    }

    pub trait RangeExtend<U>{
        fn extend(self, IN_extension: U) -> Self;
    }
    impl<T, U> RangeExtend<U> for Range<T> where T: Add<U, Output = T>{
        fn extend(self, IN_extension: U) -> Self {
            Range{
                start: self.start,
                end: self.end + IN_extension,
            }
        }
    }

    pub trait RangeShift<U>{
        fn shift(self, IN_shift: U) -> Self;
    }
    impl<T, U> RangeShift<U> for Range<T> where T: Add<U, Output = T>{
        fn shift(self, IN_shift: U) -> Self {
            Range{
                start: self.start + IN_shift,
                end: self.end
            }
        }
    }

    // Shift and Extend in one
    pub trait RangeShiftExtend<U>{
        fn shiftExtend(self, IN_startShift: U, IN_endShift: U) -> Self;
    }
    impl<T, U> RangeShiftExtend<U> for Range<T> where T: Add<U, Output = T>{
        fn shiftExtend(self, IN_startShift: U, IN_endShift: U) -> Self {
            Range{
                start: self.start + IN_startShift,
                end: self.end + IN_endShift,
            }
        }
    }
}