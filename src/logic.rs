
use crossterm::style::Color;

use crate::*;
pub struct SYS_LOGIC{

}
impl SYS_LOGIC {
    /// # Interaction manager
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// While I'm not sure how it will change exactly it does "global" interactions for now
    /// 
    /// Window system will have different way of managing those
    pub fn GAME_interact(&mut self, INi_world: &mut world::TEMPLATE_world, INi_player: &mut player::TEMPLATE_player, INi_renderer: &mut renderer::SYS_RENDERER, interactCode: GAME_interactions) {
        match interactCode {
            GAME_interactions::i_changeWorldTile => {
                INi_world
                    .w_setCell([INi_player.p_x, INi_player.p_y], 'c', Color::Black, INi_player.p_colorBg)
            }
            GAME_interactions::i_printHello => INi_renderer.r_pushText(renderer::RENDER_textItem {
                t_text: "Hello!\nHello!".to_string(),
                t_position: [0, 0],
                t_lifetime: 32,
            }),
            GAME_interactions::i_printDebug => INi_renderer.r_pushText(renderer::RENDER_textItem {
                t_text: "DEBUG".to_string(),
                t_position: [32, 32],
                t_lifetime: 16,
            }),
            GAME_interactions::i_clearWorld => INi_world.w_clearWorld(),
            GAME_interactions::i_movPlayer(dir) => INi_player.p_move(dir),
            GAME_interactions::i_NULL => {}
        }
    }
}

/// # Interactions enum
/// # DON'T RELY ON THIS
/// It will be replaced with introduction of Window system
pub enum GAME_interactions {
    i_NULL,
    i_movPlayer(u8),
    i_changeWorldTile,
    i_printHello,
    i_printDebug,
    i_clearWorld,
}