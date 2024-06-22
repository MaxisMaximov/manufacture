pub use super::*;

pub mod player;
pub mod world;

/// # Master Data struct
/// Holds every required data of the game such as player and world, soon buildings
pub struct DATA_master {
    pub DATA_player: player::TEMPLATE_player,
    pub DATA_world: world::TEMPLATE_world,
    pub DATA_textItems: Vec<IDDQD_textItem>,
    pub DATA_playerInput: logic::GAME_interactions,
    #[allow(unused)]
    DATA_cache: HashMap<String, CACHE_TYPE>,
}
impl DATA_master {
    pub fn new(IN_player: player::TEMPLATE_player) -> Self {
        Self {
            DATA_player: IN_player,
            DATA_world: world::TEMPLATE_world::new(),
            DATA_textItems: Vec::new(),
            DATA_playerInput: logic::GAME_interactions::i_NULL,
            DATA_cache: HashMap::new(),
        }
    }
    pub fn DATA_textItemCleanup(&mut self) {
        self.DATA_textItems
            .retain(|x| x.t_string != "#MARK_FOR_DELETION")
    }
}