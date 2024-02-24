
use crossterm::style::Color;

use crate::*;

/// # Game logic
pub struct SYS_LOGIC{

}
impl SYS_LOGIC {
    /// # Interaction manager
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// While I'm not sure how it will change exactly it does "global" interactions for now
    /// 
    /// Window system will have different way of managing those
    pub fn GAME_interact(&mut self, INi_data: &mut DATA_master) {
        match &INi_data.DATA_playerInput {
            GAME_interactions::i_changeWorldTile => {
                INi_data.DATA_world
                    .w_setCell(INi_data.DATA_player.p_pos, 'c', [Color::Black, INi_data.DATA_player.p_color[1]])
            }
            GAME_interactions::i_printHello => INi_data.DATA_pushTextItem(renderer::RENDER_textItem {
                t_text: format!("Hello!{NEW}Hello!", NEW = system::SYS_NEWLINE).to_string(),
                t_position: [0, 0],
                t_lifetime: 32,
            }),
            GAME_interactions::i_printDebug => INi_data.DATA_pushTextItem(renderer::RENDER_textItem {
                t_text: "DEBUG".to_string(),
                t_position: [32, 32],
                t_lifetime: 16,
            }),
            GAME_interactions::i_clearWorld => INi_data.DATA_world.w_clearWorld(),
            GAME_interactions::i_movPlayer(dir) => INi_data.DATA_player.p_move(dir),
            GAME_interactions::i_NULL => {}
        }
    }
}

/// # Interactions enum
/// # DON'T RELY ON THIS
/// It will be replaced with introduction of Window system
pub enum GAME_interactions {
    i_NULL,
    i_movPlayer(player::GAME_playerDirections),
    i_changeWorldTile,
    i_printHello,
    i_printDebug,
    i_clearWorld,
}