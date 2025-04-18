use super::*;

use constants::*;
use utils::*;

pub struct res_GridWorld{
    chunks: HashMap<Vector2, GridWorldChunk>
}
impl gmRes for res_GridWorld{
    fn new() -> Self {
        let mut idkfa_map: HashMap<Vector2, GridWorldChunk> = HashMap::new();

        for CH_X in WORLD_X_MIN..=WORLD_X_MAX {
            for CH_Y in WORLD_Y_MIN..=WORLD_Y_MAX{
                idkfa_map.insert((CH_X, CH_Y), GridWorldChunk::new());
            }
        }

        Self{
            chunks: idkfa_map
        }
    }

    fn RES_ID() -> &'static str {
        "res_GridWorld"
    }
}
impl res_GridWorld{
    pub fn getChunk(&self, IN_coords: Vector2) -> Option<&GridWorldChunk>{
        self.chunks.get(&IN_coords)
    }
    pub fn getChunkMut(&mut self, IN_coords: Vector2) -> Option<&mut GridWorldChunk>{
        self.chunks.get_mut(&IN_coords)
    }

    pub fn getChunkFromTile(&self, IN_coords: Vector2) -> Option<&GridWorldChunk>{
        self.getChunk(util_coordConvert(IN_coords).0)
    }
    pub fn getChunkFromTileMut(&mut self, IN_coords: Vector2) -> Option<&mut GridWorldChunk>{
        self.getChunkMut(util_coordConvert(IN_coords).0)
    }

    pub fn getTile(&self, IN_coords: Vector2) -> Option<&GridWorldTile>{
        let (w_chunkPos, w_tilePos) = util_coordConvert(IN_coords);

        if let Some(CHUNK) = self.getChunk(w_chunkPos){
            return Some(&CHUNK[w_tilePos])
        }
        None
    }
    pub fn getTileMut(&mut self, IN_coords: Vector2) -> Option<&mut GridWorldTile>{
        let (w_chunkPos, w_tilePos) = util_coordConvert(IN_coords);

        if let Some(CHUNK) = self.getChunkMut(w_chunkPos){
            return Some(&mut CHUNK[w_tilePos])
        }
        None
    }

    pub fn getChunkRange(&self, IN_cornerA: Vector2, IN_cornerB: Vector2) -> Vec<Vector2>{
        let mut OUT_vec = Vec::new();

        let mut w_chunkCornerA = util_coordConvert(IN_cornerA).0;
        let mut w_chunkCornerB = util_coordConvert(IN_cornerB).0;

        // Swap X coords if A is further ahead
        if w_chunkCornerA.0 > w_chunkCornerB.0 {
            (w_chunkCornerA.0, w_chunkCornerB.0) = (w_chunkCornerB.0, w_chunkCornerA.0)
        }

        // Swap Y coords if A is higher
        if w_chunkCornerA.1 > w_chunkCornerB.1 {
            (w_chunkCornerA.1, w_chunkCornerB.1) = (w_chunkCornerB.1, w_chunkCornerA.1)
        }

        // Reason for all that is to make this iterate properly
        for XPOS in w_chunkCornerA.0..=w_chunkCornerB.0{
            for YPOS in w_chunkCornerA.1..=w_chunkCornerB.1{

                if self.chunks.contains_key(&(XPOS, YPOS)){
                    OUT_vec.push((XPOS, YPOS));
                }
            }
        }

        OUT_vec
    }
}
pub struct GridWorldChunk{
    pub cells: [GridWorldTile; (CHUNK_X * CHUNK_Y) as usize],
    pub needsResprite: bool
}
impl GridWorldChunk{
    pub fn new() -> Self{
        Self{
            cells: [GridWorldTile{mat: 0}; (CHUNK_X * CHUNK_Y) as usize],
            needsResprite: false
        }
    }
}
impl Index<Vector2> for GridWorldChunk{
    type Output = GridWorldTile;

    fn index(&self, index: Vector2) -> &Self::Output {
        // Absolute offsets in the chunks
        let w_XCoord = (index.0 + CHUNK_X) % CHUNK_X;
        let w_YCoord = (index.1 + CHUNK_Y) % CHUNK_Y;
        &self.cells[(w_XCoord + w_YCoord * CHUNK_X) as usize]
    }
}
impl IndexMut<Vector2> for GridWorldChunk{
    fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
        // Absolute offsets in the chunks
        let w_XCoord = (index.0 + CHUNK_X) % CHUNK_X;
        let w_YCoord = (index.1 + CHUNK_Y) % CHUNK_Y;
        &mut self.cells[(w_XCoord + w_YCoord * CHUNK_X) as usize]
    }
}
#[derive(Clone, Copy)]
pub struct GridWorldTile{
    pub mat: u8 // Just a number for now
}