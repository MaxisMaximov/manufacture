
use std::fmt;

use crossterm::style::Color;

use crate::*;

/// # Game logic
pub struct SYS_LOGIC{

}
impl SYS_LOGIC {
    pub fn new() -> Self{
        let mut DEBUG_LOCK = SYS_debug.lock().unwrap();
        DEBUG_LOCK.DEBUG_debugStr_ADD(
            "#LOGIC_interaction",
            ".DEBUG_logic/#LOGIC_interaction",
            "",
            255
        );

        DEBUG_LOCK.DEBUG_debugStr_ADD(
            "#SSINIT_logic",
            ".DEBUG_sys/.SYS_ssInit/#SSINIT_logic",
            "",
            40
        );
        Self {}
    }
    /// # Interaction manager
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// While I'm not sure how it will change exactly it does "global" interactions for now
    /// 
    /// Window system will have different way of managing those
    pub fn GAME_interact(&mut self) {
        let mut DATA_LOCK = SYS_data.lock().unwrap();
        let idkfa_interaction = DATA_LOCK.DATA_playerInput;
        SYS_debug.lock().unwrap().DEBUG_debugStr_GET("#LOGIC_interaction").unwrap().ds_updateValues(&format!("{}", idkfa_interaction));
        match DATA_LOCK.DATA_playerInput {
            GAME_interactions::i_changeWorldTile => {
                let idkfa_pos = DATA_LOCK.DATA_player.p_pos;
                let idkfa_colors = [Color::Black, DATA_LOCK.DATA_player.p_color[1]];
                DATA_LOCK.DATA_world.w_setCell(idkfa_pos, 'c', idkfa_colors);
            }
            GAME_interactions::i_printHello => DATA_LOCK.DATA_pushTextItem("Hello!\r\nHello!", renderer::RENDER_position::POS_TL, 32),
            GAME_interactions::i_printDebug => DATA_LOCK.DATA_pushTextItem("DEBUG", renderer::RENDER_position::POS_middle, 16),
            GAME_interactions::i_clearWorld => DATA_LOCK.DATA_world.w_clearWorld(),
            GAME_interactions::i_movPlayer(dir) => {
                let idkfa_direction = dir;
                DATA_LOCK.DATA_player.p_move(&idkfa_direction)}
            GAME_interactions::i_NULL => {}
        }
    }
}

/// # Interactions enum
/// # DON'T RELY ON THIS
/// It will be replaced with introduction of Window system
#[derive(Debug, Clone, Copy)]
pub enum GAME_interactions {
    i_NULL,
    i_movPlayer(player::GAME_playerDirections),
    i_changeWorldTile,
    i_printHello,
    i_printDebug,
    i_clearWorld,
}
impl fmt::Display for GAME_interactions{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idkfa_pDir;
        let idkfa_fmt = match *self {
            Self::i_NULL => "NULL",
            Self::i_movPlayer(dir) => {idkfa_pDir = format!("movPlayer >> {}", dir); &idkfa_pDir},
            Self::i_changeWorldTile => "changeWorldTile",
            Self::i_printHello => "printHello",
            Self::i_printDebug => "printDebug",
            Self::i_clearWorld => "clearWorld"
        };
        write!(f, "{}", idkfa_fmt)
    }
}