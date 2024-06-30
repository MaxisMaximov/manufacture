use std::ops::Add;

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
pub struct TEMPLATE_player {
    pub p_pos: types::vector2,
    pub p_hp: u16,
    pub p_chunk: types::vector2,
    pub p_color: types::colorSet
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
            p_pos: (10, 10),
            p_hp: vars::PLAYER::PLAYER_BASE_HP,
            p_chunk: (2, 2),
            p_color: (Color::White, Fp_playerColor) }
    }

    pub fn p_move(&mut self, dir: &logic::GAME_playerDirections, stepSize: usize) {
        match dir {
            logic::GAME_playerDirections::DIR_up => {
                self.p_pos.1 = self.p_pos.1.saturating_sub(stepSize);
            }
            logic::GAME_playerDirections::DIR_down => {
                self.p_pos.1 = self.p_pos.1.add(stepSize).clamp(0, vars::WORLD::GENERAL::GRID_Y);
            }
            logic::GAME_playerDirections::DIR_left => {
                self.p_pos.0 = self.p_pos.0.saturating_sub(stepSize);
            }
            logic::GAME_playerDirections::DIR_right => {
                self.p_pos.0 = self.p_pos.0.add(stepSize).clamp(0, vars::WORLD::GENERAL::GRID_X);
            }
        }
        // Update current chunk         // I hate when small changes like this comment flag the whole file as Modified.
        self.p_updateChunk()
    }

    pub fn p_updateChunk(&mut self){
        self.p_chunk.0 = self.p_pos.0 / vars::WORLD::GENERAL::CHUNK_X;
        self.p_chunk.1 = self.p_pos.1 / vars::WORLD::GENERAL::CHUNK_Y;
    }
}

/// # Player color "enum"
/// ## Disclaimer:
/// Is only for Player 1-4 colors
const GAME_playerColors: [Color;4] = [
    Color::Cyan,
    Color::Green,
    Color::Yellow,
    Color::Rgb {r: 255, g: 153, b: 0}
];