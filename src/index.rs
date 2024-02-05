use std::ops::Range;

use crossterm::style::Color;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::system;

// region: Structs

/// # Player struct
/// Use 1-4 in `fp_playerNum` when initializing to change the color
/// 
/// Available colors:
/// * 1 - Cyan
/// * 2 - Green
/// * 3 - Yellow
/// * 4 - Orange
/// 
/// # Custom colors
/// To instead use custom colors set `fp_playerNum` to 0 and `fp_color` to [`Color::Rgb`]
pub struct TEMPLATE_player {
    pub p_x: usize,
    pub p_y: usize,
    pub p_chunkX: usize,
    pub p_chunkY: usize,
    pub p_colorChar: Color,
    pub p_colorBg: Color,
}
impl TEMPLATE_player {
    pub fn new(INp_playerNum: usize, INp_color: Option<Color>) -> Self{
        let Fp_playerColor: Color = if INp_playerNum == 0{
            INp_color.unwrap()
        }
        else {
            GAME_playerColors[INp_playerNum]
        };
        TEMPLATE_player {
            p_x: 11,
            p_y: 55,
            p_chunkX: 0,
            p_chunkY: 0,
            p_colorChar: Color::White,
            p_colorBg: Fp_playerColor }
    }
    pub fn p_move(&mut self, dir: u8) {
        match dir {
            0 => {
                // Up
                if self.p_y == 0 {
                    return;
                }
                self.p_y -= 1
            }
            1 => {
                //Down
                if self.p_y == (system::SYS_GRID_Y - 1) {
                    return;
                }
                self.p_y += 1
            }
            2 => {
                //Left
                if self.p_x == 0 {
                    return;
                }
                self.p_x -= 1
            }
            3 => {
                //Right
                if self.p_x == (system::SYS_GRID_X - 1) {
                    return;
                }
                self.p_x += 1
            }
            _ => {}
        }
    }
    pub fn p_updateChunkPos(&mut self){
        self.p_chunkX = (self.p_x / system::SYS_CHUNK_X);
        self.p_chunkY = (self.p_y / system::SYS_CHUNK_Y);
    }
}

/// # World/Render Buffer Cell
/// 
/// Values:
/// 
/// * Character
/// * Color for character
/// * Color for background
pub struct TEMPLATE_wrCell {
    pub c_char: char,
    pub c_colChr: Color,
    pub c_colBg: Color,
}
impl TEMPLATE_wrCell{
    pub fn new() -> Self{
        TEMPLATE_wrCell { c_char: ' ', c_colChr: Color::White, c_colBg: Color::Black }
    }
    pub fn newDummy() -> Self{
        TEMPLATE_wrCell { c_char: '0', c_colChr: Color::Black, c_colBg: Color::White }
    }
}
impl Copy for TEMPLATE_wrCell {}
impl Clone for TEMPLATE_wrCell {
    fn clone(&self) -> Self {
        TEMPLATE_wrCell {
            c_char: self.c_char,
            c_colChr: self.c_colChr,
            c_colBg: self.c_colBg,
        }
    }
}

/// # World Chunk struct
/// For now holds only cells, dunno what else to add to it
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
impl Copy for TEMPLATE_wChunk {}
impl Clone for TEMPLATE_wChunk {
    fn clone(&self) -> Self {
        TEMPLATE_wChunk { ch_cells: self.ch_cells }
    }
}

/// # "Textbox" struct
/// Lets you paste a text somewhere in the game screen
/// 
/// # DO NOT RELY ON THIS
/// It'll be replaced in favor of Window system
/// 
/// # Warning
/// The Renderer doesn't check if the text overflows the X position yet, only if it's outside the buffer
/// 
/// So be careful where and what you write
pub struct RENDER_textItem{
    pub t_position: [usize; 2],
    pub t_text: String,
    pub t_lifetime: u16
}


/// # Game world
/// For now relies on `SYS_WORLD` values in `system.rs`
/// 
/// TODO: Make it handle buildings and chunks
/// 
/// `w_clearWorld` function is for debug purposes for now
pub struct TEMPLATE_world {
    pub w_chunks: [TEMPLATE_wChunk; (system::SYS_WORLD_X * system::SYS_WORLD_Y)],
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
    fn w_util_circleIter(&self, INw_RNG: &mut ThreadRng, INw_initPos: [usize;2], INw_iters: usize, INw_size: Range<usize>) -> Vec<[usize; 2]>{

        let mut w_genLakeTiles: Vec<[usize; 2]> = Vec::new();

        // Set variables to be used by iterations
        let mut w_iterCoords: [usize; 2] = INw_initPos;
        let mut w_nextIterCoords: [usize; 2];
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
                        w_genLakeTiles.push(w_cellPos);
                        continue;
                    }
                    // If it's not in rhomb or radius then skip
                    if w_cellPos[0].abs_diff(w_iterCoords[0]).pow(2) + w_cellPos[1].abs_diff(w_iterCoords[1]).pow(2) > w_iterRadius.pow(2){
                        continue;
                    }
                    // If all previous checks passed that means it's valid
                    w_genLakeTiles.push(w_cellPos)
                }
            }

            // Sort and remove duplicate coords to not waste memory
            w_genLakeTiles.sort();
            w_genLakeTiles.dedup();

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
        
        return w_genLakeTiles;
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
        let mut w_genLakeTiles: Vec<[usize; 2]> = Vec::new();
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
            self.w_setCell(COORDS, 'W', Color::White, Color::Blue)
        }

        // Forests
        let mut w_genForestTiles: Vec<[usize; 2]> = Vec::new();
        for _ in 0..w_RNG.gen_range(system::WORLD_FOREST_Q){
            // Set values for given forest
            let w_forestRandomX:usize = w_RNG.gen_range(8..system::SYS_GRID_X - 8);
            let w_forestRandomY:usize = w_RNG.gen_range(8..system::SYS_GRID_Y - 8);
            let w_forestIters:usize = w_RNG.gen_range(system::WORLD_POND_ITERS);
            
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
            if self.w_getCell(COORDS).c_char != ' '{
                continue;
            }
            self.w_setCell(COORDS, 'F', Color::White, Color::DarkGreen)
        }


    }
    /// # Calculate position in the world
    /// Takes `[X, Y]` coords as input and outputs `[ChunkIndex, CellIndex]`
    pub fn w_calcPosIndex(&self, INw_position: [usize;2]) -> [usize;2]{
        return [
            ((INw_position[0] / system::SYS_CHUNK_X)) + ((INw_position[1] / system::SYS_CHUNK_Y)) * system::SYS_WORLD_X,
            (INw_position[0] % system::SYS_CHUNK_X) + (INw_position[1] % system::SYS_CHUNK_Y) * system::SYS_CHUNK_X
        ];
    }

    /// # Get a slice of the world of `[X, Y]` size centered on chunk
    /// Returns array of chunk references
    /// 
    /// Any area that is out of bounds gets filled with Dummy Chunks
    pub fn w_returnChunkArray(&self, w_iterCoords: [usize;2], INw_size: [usize; 2]) -> Vec<&TEMPLATE_wChunk>{

        let w_radius = [
            INw_size[0] / 2,
            INw_size[1] / 2
        ];

        let w_startPosition = [
                w_iterCoords[0].saturating_sub(INw_size[0] / 2),
                w_iterCoords[1].saturating_sub(INw_size[1] / 2)
            ];
        let mut OUTw_chunkVec: Vec<&TEMPLATE_wChunk> = vec![&self.w_dummyChunk; INw_size[0] * INw_size[1]];
        let mut WORLDYPOS = 0;
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

    pub fn w_setCell(&mut self, INw_position: [usize;2], INw_character: char, INw_colorChar: Color, INw_colorBg: Color) {
        let w_workingPosition = self.w_calcPosIndex(INw_position);
        self.w_chunks[w_workingPosition[0]].ch_cells[w_workingPosition[1]] = TEMPLATE_wrCell{c_char: INw_character, c_colChr: INw_colorChar, c_colBg: INw_colorBg};
    }

    pub fn w_getCell(&self, INw_position: [usize;2]) -> &TEMPLATE_wrCell{
        let w_workingPos = self.w_calcPosIndex(INw_position);
        return &self.w_chunks[w_workingPos[0]].ch_cells[w_workingPos[1]];
    }

    pub fn w_clearWorld(&mut self) {
        self.w_chunks.fill(TEMPLATE_wChunk::new())
    }
}

// endregion: Structs

// region: Enums

/// # Interactions enum
/// # DON'T RELY ON THIS
/// It will be replaced with introduction of Window system
pub enum GAME_interactions {
    i_changeWorldTile,
    i_printHello,
    i_printDebug,
    i_clearWorld,
}

/// # Player color "enum"
/// ## Disclaimer:
/// Is only for Player 1-4 colors
pub const GAME_playerColors: [Color;4] = [
    Color::Cyan,
    Color::Green,
    Color::Yellow,
    Color::Rgb {r: 255, g: 153, b: 0}
];

// endregion: Enums