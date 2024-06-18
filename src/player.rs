use std::{fmt, ops::Add};

use crossterm::style::Color;

use crate::*;

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
    pub p_pos: TYPE::vector2,
    pub p_chunk: TYPE::vector2,
    pub p_color: TYPE::colorSet
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
            p_chunk: (2, 2),
            p_color: (Color::White, Fp_playerColor) }
    }

    pub fn p_move(&mut self, dir: &GAME_playerDirections) {
        match dir {
            GAME_playerDirections::DIR_up => {
                self.p_pos.1 = self.p_pos.1.saturating_sub(1);
            }
            GAME_playerDirections::DIR_down => {
                self.p_pos.1 = self.p_pos.1.add(1).clamp(0, WORLD::GENERAL::WORLD_GRID_Y);
            }
            GAME_playerDirections::DIR_left => {
                self.p_pos.0 = self.p_pos.0.saturating_sub(1);
            }
            GAME_playerDirections::DIR_right => {
                self.p_pos.0 = self.p_pos.0.add(1).clamp(0, WORLD::GENERAL::WORLD_GRID_X);
            }
        }
        // Update current chunk         // I hate when small changes like this comment flag the whole file as Modified.
        self.p_chunk.0 = self.p_pos.0 / WORLD::GENERAL::WORLD_CHUNK_X;
        self.p_chunk.1 = self.p_pos.1 / WORLD::GENERAL::WORLD_CHUNK_Y;
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

/// # Player direction enum
/// This exists solely for readbility
///
/// But also if I'd like to have more "advanced" movement
#[derive(Debug, Clone, Copy)]
pub enum GAME_playerDirections {
    DIR_up,
    DIR_down,
    DIR_left,
    DIR_right
}
impl fmt::Display for GAME_playerDirections{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idkfa_string = match *self {
            GAME_playerDirections::DIR_up => "Up",
            GAME_playerDirections::DIR_down => "Down",
            GAME_playerDirections::DIR_left => "Left",
            GAME_playerDirections::DIR_right => "Right"
        };
        write!(f, "{}", idkfa_string)
    }
}