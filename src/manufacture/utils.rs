use types::Vector2;
use vars::*;

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