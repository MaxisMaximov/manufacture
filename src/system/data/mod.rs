pub use super::*;

pub mod player;
pub mod world;

/// # Master Data struct
/// Holds every required data of the game such as player and world, soon buildings
pub struct DATA_master {
    pub DATA_player: player::TEMPLATE_player,
    pub DATA_world: world::TEMPLATE_world,
    pub DATA_playerInput: logic::GAME_interactions,
}
impl DATA_master {
    pub fn new(IN_player: player::TEMPLATE_player) -> Self {
        Self {
            DATA_player: IN_player,
            DATA_world: world::TEMPLATE_world::new(),
            DATA_playerInput: logic::GAME_interactions::i_NULL,
        }
    }
}