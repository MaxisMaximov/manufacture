use std::ops::{Index, IndexMut, Range};

use crossterm::style::Color;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::*;

/// # World Cell
/// 
/// Values:
/// 
/// * Character
/// * Colors for character and background
///
/// TODO: Make it store buildings as well
#[derive(Clone, Copy)]
pub struct TEMPLATE_wrCell {
    pub c_char: char,
    pub c_color: system::cellColors,
}
impl TEMPLATE_wrCell{
    pub fn new() -> Self{
        TEMPLATE_wrCell { c_char: ' ', c_color: [Color::White, Color:: Black] }
    }
    pub fn newDummy() -> Self{
        TEMPLATE_wrCell { c_char: '0', c_color: [Color::Black, Color:: White] }
    }
}

/// # World Chunk struct
/// For now holds only cells, dunno what else to add to it
#[derive(Clone, Copy)]
pub struct TEMPLATE_wChunk {
    pub ch_cells: [TEMPLATE_wrCell; system::SYS_CHUNK_X * system::SYS_CHUNK_Y]
}
impl TEMPLATE_wChunk {
    pub fn new() -> Self {
        TEMPLATE_wChunk { ch_cells: [TEMPLATE_wrCell::new(); system::SYS_CHUNK_X * system:: SYS_CHUNK_Y] }
    }
    pub fn newDummy() -> Self {
        TEMPLATE_wChunk { ch_cells: [TEMPLATE_wrCell::newDummy(); system::SYS_CHUNK_X * system:: SYS_CHUNK_Y]}
    }
}
impl Index<system::coords> for TEMPLATE_wChunk{
    type Output = TEMPLATE_wrCell;
    fn index(&self, index: system::coords) -> &Self::Output {
        &self.ch_cells[index[0] + index[1] * system::SYS_CHUNK_X]
    }
}
impl IndexMut<system::coords> for TEMPLATE_wChunk{
    fn index_mut(&mut self, index: system::coords) -> &mut Self::Output {
        &mut self.ch_cells[index[0] + index[1] * system::SYS_CHUNK_X]
    }
}

/// # Game world
/// For now relies on `SYS_WORLD` values in `system.rs`
/// 
/// TODO: Make it handle buildings and chunks
/// 
/// `w_clearWorld` function is for debug purposes for now
pub struct TEMPLATE_world {
    pub w_chunks: [TEMPLATE_wChunk; system::SYS_WORLD_X * system::SYS_WORLD_Y],
    pub w_dummyChunk: TEMPLATE_wChunk
}
impl TEMPLATE_world {
    pub fn new() -> Self{
        TEMPLATE_world { 
            w_chunks: [TEMPLATE_wChunk::new(); system::SYS_WORLD_X*system::SYS_WORLD_Y],
            w_dummyChunk: TEMPLATE_wChunk::newDummy()
         }
    }

    /// # Circlegen iteration
    /// Each iteration generates it's circle picks coord for next iteration within it's range
    fn w_util_circleIter(&self, INw_RNG: &mut ThreadRng, INw_initPos: system::coords, INw_iters: usize, INw_size: Range<usize>) -> Vec<system::coords>{

        let mut w_genCircleTiles: Vec<system::coords> = Vec::new();

        // Set variables to be used by iterations
        let mut w_iterCoords: system::coords = INw_initPos;
        let mut w_nextIterCoords: system::coords;
        let mut w_iterSize: usize = INw_RNG.gen_range(INw_size.clone());
        let mut w_iterRadius: usize = w_iterSize / 2;

        let mut w_nextIterAreaX: Range<usize>;
        let mut w_nextIterAreaY: Range<usize>;

        for _ in 0..INw_iters{
            // Find coords of tiles where the iteration has influence
            let w_circleStartingPosition = [w_iterCoords[0].saturating_sub(w_iterRadius), w_iterCoords[1].saturating_sub(w_iterRadius)];
            for CELLY in 0..w_iterSize{
                // If iterator is out of Y bounds of the world, don't iterate over the rest
                if w_circleStartingPosition[1] + CELLY >= system::SYS_GRID_Y{
                    break;
                }
                for CELLX in 0..w_iterSize{
                    // If iterator is out of X bounds of the world, don't iterate over the rest
                    if (w_circleStartingPosition[0] + CELLX) >= system::SYS_GRID_X{
                        break;
                    }
                    
                    // Now checks for the cell itself
                    let w_cellPos = [w_circleStartingPosition[0] + CELLX, w_circleStartingPosition[1] + CELLY];
                    
                    // If it's inside the rhomb inside the circle it's guaranteed to be valid
                    if w_cellPos[0].abs_diff(w_iterCoords[0]) + w_cellPos[1].abs_diff(w_iterCoords[1]) <= w_iterRadius{
                        w_genCircleTiles.push(w_cellPos);
                        continue;
                    }
                    // If it's not in rhomb or radius then skip
                    if w_cellPos[0].abs_diff(w_iterCoords[0]).pow(2) + w_cellPos[1].abs_diff(w_iterCoords[1]).pow(2) > w_iterRadius.pow(2){
                        continue;
                    }
                    // If all previous checks passed that means it's valid
                    w_genCircleTiles.push(w_cellPos)
                }
            }

            // Sort and remove duplicate coords to not waste memory
            w_genCircleTiles.sort();
            w_genCircleTiles.dedup();

            // Find coords for next iteration
            w_nextIterAreaX = w_iterCoords[0].saturating_sub(w_iterRadius)..(w_iterCoords[0] + w_iterRadius).clamp(0, system::SYS_GRID_X);
            w_nextIterAreaY = w_iterCoords[1].saturating_sub(w_iterRadius)..(w_iterCoords[1] + w_iterRadius).clamp(0, system::SYS_GRID_X);
            loop {
                w_nextIterCoords = [INw_RNG.gen_range(w_nextIterAreaX.clone()), INw_RNG.gen_range(w_nextIterAreaY.clone())];
                if w_nextIterCoords[0].abs_diff(w_iterCoords[0]).pow(2) + w_nextIterCoords[1].abs_diff(w_iterCoords[1]).pow(2) > w_iterRadius.pow(2){
                    continue;
                }
                break;
            }

            // Set variables for next iteration
            w_iterCoords = w_nextIterCoords;
            w_iterSize = INw_RNG.gen_range(INw_size.clone());
            w_iterRadius = w_iterSize / 2;
        }
        
        return w_genCircleTiles;
    }

    pub fn w_generateRandom(&mut self){
        let mut w_RNG = thread_rng();
        // # PIPELINE (NOT FINAL):
        // 1. Generate Lakes
        // - Pick random sparse points
        // - Circle fill random distance
        // - Inside those circles, repeat random amount of times
        // 2. Generate Cliffs/walls
        // - Pick random sparse points
        // - If point is on water, abort
        // - Pick random axis, can't be more than 75d if it's at end of a cliff already
        // - Extrude by said axis by random amount
        // - Repeat 2-3 times
        // 3. Generate Forests
        // - Same thing as lakes, except don't overlap any Cliff or Water tiles

        // Ponds
        // Vector with final coordinates for water tiles to replace them all at once instead of 1 by 1
        let mut w_genLakeTiles: Vec<system::coords> = Vec::new();
        for _ in 0..w_RNG.gen_range(system::WORLD_POND_Q){
            // Set values for given lake
            let w_lakeRandomX:usize = w_RNG.gen_range(8..system::SYS_GRID_X - 8);
            let w_lakeRandomY:usize = w_RNG.gen_range(8..system::SYS_GRID_Y - 8);
            let w_lakeIters:usize = w_RNG.gen_range(system::WORLD_POND_ITERS);
            
            // Let the iterator handle the rest
            w_genLakeTiles.extend(self.w_util_circleIter(&mut w_RNG,
                [w_lakeRandomX, w_lakeRandomY],
                w_lakeIters,
                system::WORLD_POND_SIZE));
            
            // Sort and remove duplicates
            w_genLakeTiles.sort();
            w_genLakeTiles.dedup();
        }
        for COORDS in w_genLakeTiles{
            self[COORDS] = TEMPLATE_wrCell{c_char: 'W', c_color: [Color::White, Color::Blue]}
        }

        // Forests
        let mut w_genForestTiles: Vec<system::coords> = Vec::new();
        for _ in 0..w_RNG.gen_range(system::WORLD_FOREST_Q){
            // Set values for given forest
            let w_forestRandomX:usize = w_RNG.gen_range(8..system::SYS_GRID_X - 8);
            let w_forestRandomY:usize = w_RNG.gen_range(8..system::SYS_GRID_Y - 8);
            let w_forestIters:usize = w_RNG.gen_range(system::WORLD_FOREST_ITERS);
            
            // Let the iterator handle the rest
            w_genForestTiles.extend(self.w_util_circleIter(&mut w_RNG,
                [w_forestRandomX, w_forestRandomY],
                w_forestIters,
                system::WORLD_FOREST_SIZE));
            
            // Sort and remove duplicates
            w_genForestTiles.sort();
            w_genForestTiles.dedup();
        }
        for COORDS in w_genForestTiles{
            // Skip cells that are already occupied by lakes
            if self[COORDS].c_char != ' '{
                continue;
            }
            self[COORDS] = TEMPLATE_wrCell{c_char: 'F', c_color: [Color::White, Color::DarkGreen]}
        }


    }
    /// # Calculate position in the world
    /// Takes `[X, Y]` coords as input and outputs `[ChunkIndex, CellIndex]`
    pub fn w_calcPosIndex(&self, INw_position: system::coords) -> system::coords{
        return [
            ((INw_position[0] / system::SYS_CHUNK_X)) + ((INw_position[1] / system::SYS_CHUNK_Y)) * system::SYS_WORLD_X,
            (INw_position[0] % system::SYS_CHUNK_X) + (INw_position[1] % system::SYS_CHUNK_Y) * system::SYS_CHUNK_X
        ];
    }

    /// # Get a slice of the world of `[X, Y]` size centered on chunk
    /// Returns array of chunk references
    /// 
    /// Any area that is out of bounds gets filled with Dummy Chunks
    pub fn w_returnChunkArray(&self, w_iterCoords: system::coords, INw_size: system::coords) -> Vec<&TEMPLATE_wChunk>{

        let w_radius = [INw_size[0] / 2, INw_size[1] / 2];

        let w_startPosition = [
                w_iterCoords[0].saturating_sub(INw_size[0] / 2),
                w_iterCoords[1].saturating_sub(INw_size[1] / 2)
            ];
        let mut OUTw_chunkVec: Vec<&TEMPLATE_wChunk> = vec![&self.w_dummyChunk; INw_size[0] * INw_size[1]];
        let mut WORLDYPOS = 0;

        // This is another mess to fix.
        for YPOS in (
                w_radius[1] - w_startPosition[1].abs_diff(w_iterCoords[1])
                )..=(
                    w_radius[1] - w_startPosition[1].abs_diff(w_iterCoords[1]) + w_startPosition[1].abs_diff((w_iterCoords[1] + w_radius[1]).clamp(0, system::SYS_WORLD_Y - 1))
                ){
                let mut WORLDXPOS: usize = 0;
                for XPOS in (
                    w_radius[0] - w_startPosition[0].abs_diff(w_iterCoords[0])
                    )..=(
                        w_radius[0] - w_startPosition[0].abs_diff(w_iterCoords[0]) + w_startPosition[0].abs_diff((w_iterCoords[0] + w_radius[0]).clamp(0, system::SYS_WORLD_X - 1))
                    ){
                    OUTw_chunkVec[XPOS + YPOS * INw_size[0]] = &self.w_chunks[w_startPosition[0] + WORLDXPOS + (w_startPosition[1] + WORLDYPOS) * system::SYS_WORLD_X];
                    WORLDXPOS += 1
                }
                WORLDYPOS += 1
        }
        return OUTw_chunkVec;
    }

    pub fn w_clearWorld(&mut self) {
        self.w_chunks.fill(TEMPLATE_wChunk::new())
    }
}

impl Index<system::coords> for TEMPLATE_world{
    type Output = TEMPLATE_wrCell;

    fn index(&self, index: system::coords) -> &Self::Output {
        &self.w_chunks[
            index[0]/system::SYS_CHUNK_X + 
            index[1]/system::SYS_CHUNK_Y * system::SYS_WORLD_X]
                [[index[0]%system::SYS_CHUNK_X, index[1]%system::SYS_CHUNK_Y]]
    }
}
impl IndexMut<system::coords> for TEMPLATE_world{
    fn index_mut(&mut self, index: system::coords) -> &mut Self::Output {
        &mut self.w_chunks[
            index[0]/system::SYS_CHUNK_X + 
            index[1]/system::SYS_CHUNK_Y * system::SYS_WORLD_X]
                [[index[0]%system::SYS_CHUNK_X, index[1]%system::SYS_CHUNK_Y]]
    }
}