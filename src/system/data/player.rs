use super::*;

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
pub struct obj_player {
    pub hp: u16,
    pub loc: types::vector2,
    pub chunk: types::vector2,
    pub color: types::colorSet,

    pub inv: Vec<u8>,
    pub invIndex: usize
}
impl obj_player {
    pub fn new(IN_playerNum: usize, IN_color: Option<Color>) -> Self{
        let Fp_playerColor: Color = if IN_playerNum == 0{
            IN_color.unwrap()
        }
        else {
            player_colors[IN_playerNum]
        };
        Self {
            hp: vars::PLAYER::PLAYER_BASE_HP,
            loc: (10, 10),
            chunk: (2, 2),
            color: (Color::White, Fp_playerColor),

            inv: vec![0; vars::PLAYER::PLAYER_INV_SIZE],
            invIndex: 0,
        }
    }

    pub fn walk(&mut self, IN_dir: &logic::playerDirections, IN_stepSize: usize) {
        match IN_dir {
            logic::playerDirections::up => {
                self.loc.1 = self.loc.1.saturating_sub(IN_stepSize);
            }
            logic::playerDirections::down => {
                self.loc.1 = (self.loc.1 + IN_stepSize).clamp(0, vars::WORLD::GENERAL::GRID_Y);
            }
            logic::playerDirections::left => {
                self.loc.0 = self.loc.0.saturating_sub(IN_stepSize);
            }
            logic::playerDirections::right => {
                self.loc.0 = (self.loc.0 + IN_stepSize).clamp(0, vars::WORLD::GENERAL::GRID_X);
            }
        }
        // Update current chunk         // I hate when small changes like this comment flag the whole file as Modified.
        self.updateChunk()
    }

    pub fn updateChunk(&mut self){
        self.chunk.0 = self.loc.0 / vars::WORLD::GENERAL::CHUNK_X;
        self.chunk.1 = self.loc.1 / vars::WORLD::GENERAL::CHUNK_Y;
    }

    /// 0 - Backward | 1 - Forward
    pub fn invSelect(&mut self, IN_dir: bool){

        // Forward
        if self.invIndex < vars::PLAYER::PLAYER_INV_SIZE && IN_dir{
            self.invIndex += 1;
            return;
        }

        // Backward
        if self.invIndex > 0{
            self.invIndex -= 1
        }
    }

    /// 0 - Decr | 1 - Incr
    pub fn invMod(&mut self, IN_op: bool){

        // Incr
        if self.inv[self.invIndex] < 255 && IN_op{
            self.inv[self.invIndex] += 1;
            return;
        }

        // Decr
        if self.inv[self.invIndex] > 0{
            self.inv[self.invIndex] -= 1;
            return;
        }
    }

    /// 0 - "Remove" | 1 - Add
    /// By those I mean Remove - Set 0 | Add - Set 255
    pub fn invAddDel(&mut self, IN_op: bool){

        if IN_op{ // Add
            self.inv[self.invIndex] = 255;
            return;
        }
        // "Remove"
        self.inv[self.invIndex] = 0
    }
}

/// # Player color "enum"
/// ## Disclaimer:
/// Is only for Player 1-4 colors
const player_colors: [Color;4] = [
    Color::Cyan,
    Color::Green,
    Color::Yellow,
    Color::Rgb {r: 255, g: 153, b: 0}
];