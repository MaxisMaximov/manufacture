pub use super::*;

pub mod player;
pub mod world;

/// # Master Data struct
/// Holds every required data of the game such as player and world, soon buildings
pub struct data_master {
    pub player: player::obj_player,
    pub world: world::world_master,
    pub playerInput: logic::interactions,
}
impl data_master {
    pub fn new(IN_player: player::obj_player) -> Self {
        Self {
            player: IN_player,
            world: world::world_master::new(),
            playerInput: logic::interactions::NULL,
        }
    }
}