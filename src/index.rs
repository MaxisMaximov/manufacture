use crossterm::style::Color;

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
            p_x: 8,
            p_y: 8,
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
/// For now relies on `SYS_GRID` values in `system.rs`
/// 
/// TODO: Make it handle buildings and chunks
/// 
/// `w_clearWorld` function is for debug purposes for now
pub struct TEMPLATE_world {
    pub w_chunks: [TEMPLATE_wChunk; (system::SYS_WORLD_X * system::SYS_WORLD_Y)],
}
impl TEMPLATE_world {
    pub fn new() -> Self{
        TEMPLATE_world { 
            w_chunks: [TEMPLATE_wChunk::new(); system::SYS_WORLD_X*system::SYS_WORLD_Y],
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
    /// Auto shifts the origin if the size exceeds bounds
    pub fn w_returnChunkArray(&self, INw_centerPos: [usize;2], INw_size: [usize; 2]) -> Vec<&TEMPLATE_wChunk>{
        let mut w_actualPos = INw_centerPos;

        while w_actualPos[0].checked_sub(INw_size[0] / 2).is_none() {
            w_actualPos[0] += 1;
        }
        while w_actualPos[0] + w_actualPos[0] / 2 > system::SYS_WORLD_X {
            w_actualPos[0] -= 1;
        }
        while w_actualPos[1].checked_sub(INw_size[1] / 2).is_none() {
            w_actualPos[1] += 1;
        }
        while w_actualPos[1] + w_actualPos[1] / 2 > system::SYS_WORLD_Y {
            w_actualPos[1] -= 1;
        }

        let mut w_chunkIndexStart = (w_actualPos[0] + w_actualPos[1] * system::SYS_WORLD_X) - (INw_size[0] / 2) - (INw_size[1] / 2) * system::SYS_WORLD_X;
        let mut OUTw_chunkVec: Vec<&TEMPLATE_wChunk> = vec![&self.w_chunks[0]; INw_size[0] * INw_size[1]];
        for YPOS in 0..INw_size[1]{
            for XPOS in 0..INw_size[0]{
                OUTw_chunkVec[XPOS + YPOS * INw_size[0]] = &self.w_chunks[w_chunkIndexStart + XPOS];
            }
            w_chunkIndexStart += system::SYS_WORLD_X;
        }
        return OUTw_chunkVec;
    }

    pub fn w_setCell(&mut self, INw_position: [usize;2], INw_character: char, INw_colorChar: Color, INw_colorBg: Color) {
        let w_workingPosition = self.w_calcPosIndex(INw_position);
        self.w_chunks[w_workingPosition[0]].ch_cells[w_workingPosition[1]] = TEMPLATE_wrCell{c_char: INw_character, c_colChr: INw_colorChar, c_colBg: INw_colorBg};
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