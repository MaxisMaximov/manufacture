use std::ops::{Index, IndexMut};

use super::*;

use vars::*;

mod event;
pub use event::*;

pub struct res_PInput{
    pub res: KeyEvent
}
impl gmRes for res_PInput{
    fn new() -> Self {
        Self{
            res: KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
        }
    }

    fn RES_ID() -> &'static str {
        "res_PInput"
    }
}

pub struct res_DeltaT{
    pub res: Duration
}
impl gmRes for res_DeltaT{
    fn new() -> Self {
        Self{
            res: Duration::from_secs(0)
        }
    }

    fn RES_ID() -> &'static str {
        "res_DeltaT"
    }
}

pub struct res_PID{
    pub res: HashMap<gmID, gmID> // PID, gmObjID
}
impl gmRes for res_PID{
    fn new() -> Self {
        Self{
            res: HashMap::new()
        }
    }

    fn RES_ID() -> &'static str {
        "res_PID"
    }
}

pub struct res_GridWorld{
    chunks: HashMap<Vector2, GridWorldChunk>
}
impl gmRes for res_GridWorld{
    fn new() -> Self {
        let mut idkfa_map: HashMap<(isize, isize), GridWorldChunk> = HashMap::new();

        for CH_X in WORLD_X_MIN..WORLD_X_MAX {
            for CH_Y in WORLD_Y_MIN..WORLD_Y_MAX{
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
        self.getChunk((IN_coords.0 / CHUNK_X, IN_coords.1 / CHUNK_Y))
    }
    pub fn getChunkFromTileMut(&mut self, IN_coords: Vector2) -> Option<&mut GridWorldChunk>{
        self.getChunkMut((IN_coords.0 / CHUNK_X, IN_coords.1 / CHUNK_Y))
    }
    pub fn getTile(&self, IN_coords: Vector2) -> Option<&GridWorldTile>{
        let w_chunkCoords: Vector2 = (IN_coords.0 / CHUNK_X, IN_coords.1 / CHUNK_Y);
        let w_tileCoords: Vector2 =  (IN_coords.0 % CHUNK_X, IN_coords.1 % CHUNK_Y);

        if let Some(CHUNK) = self.getChunk(w_chunkCoords){
            return Some(&CHUNK[w_tileCoords])
        }
        None
    }
    pub fn getTileMut(&mut self, IN_coords: Vector2) -> Option<&mut GridWorldTile>{
        let w_chunkCoords: Vector2 = (IN_coords.0 / CHUNK_X, IN_coords.1 / CHUNK_Y);
        let w_tileCoords: Vector2 =  (IN_coords.0 % CHUNK_X, IN_coords.1 % CHUNK_Y);

        if let Some(CHUNK) = self.getChunkMut(w_chunkCoords){
            return Some(&mut CHUNK[w_tileCoords])
        }
        None
    }
}
pub struct GridWorldChunk{
    cells: [GridWorldTile; (CHUNK_X * CHUNK_Y) as usize]
}
impl GridWorldChunk{
    pub fn new() -> Self{
        Self{
            cells: [GridWorldTile{mat: 0}; (CHUNK_X * CHUNK_Y) as usize]
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