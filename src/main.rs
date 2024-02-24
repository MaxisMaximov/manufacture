use crossterm::terminal::enable_raw_mode;

use std::{collections::HashMap, thread::sleep, time::{Duration, Instant}};

mod logic;
mod player;
mod world;
mod input;
mod renderer;
mod system;

// START HERE
fn main() {
    enable_raw_mode().unwrap();

    // Initialize the Data container
    let mut SYS_data = DATA_master{
        DATA_player: player::TEMPLATE_player::new(1, None),
        DATA_world: world::TEMPLATE_world::new(),
        DATA_debug: String::new(),
        DATA_textItems: Vec::new(),
        DATA_playerInput: logic::GAME_interactions::i_NULL,
        DATA_cache: HashMap::new()
    };

    // Generate new world
    SYS_data.DATA_world.w_generateRandom();

    // Initialize the subsystems
    let mut SYS_subsystems = SUBSYSTEM_master{
        SUBSYSTEM_logic: logic::SYS_LOGIC{},
        SUBSYSTEM_renderer: renderer::SYS_RENDERER::new(),
        SUBSYSTEM_input: input::SYS_INPUT{}
    };

    // # THE GAME LOOP
    'GAME_loop: loop {

        // Clear debug string
        SYS_data.DATA_debug.clear();

        // Start the timer
        let loopStart: Instant = Instant::now();

        // Set next Player input to process
        SYS_subsystems.SUBSYSTEM_input.SYS_HANDLER_input(&mut SYS_data);

        // Process the input
        SYS_subsystems.SUBSYSTEM_logic.GAME_interact(&mut SYS_data);

        // Render everything
        SYS_subsystems.SUBSYSTEM_renderer.SYS_HANDLER_renderGame(&mut SYS_data);

        // Log how long it took to process everything
        let loop_elapsedTime: Duration = loopStart.elapsed();
        if loop_elapsedTime < system::SYS_TICKTIME {
            SYS_data.DATA_debug.push_str(&format!(
                "Too Fast! | {:?}{NEW}Target speed: {:?}",
                loop_elapsedTime, system::SYS_TICKTIME, NEW = system::SYS_NEWLINE
            ));
            println!("{}", SYS_data.DATA_debug);
            sleep(system::SYS_TICKTIME - loop_elapsedTime)
        } else {
            SYS_data.DATA_pushDebugStr(format!("Too slow! | {:?}", loop_elapsedTime));
            println!("{}", SYS_data.DATA_debug);
        }

    }
}

/// # Master Data struct
/// Holds every required data of the game such as player, world and Debug data
/// 
/// Cache must be gotten/added/removed through `DATA_cacheData` functions
/// 
/// I do not trust myself to do it correctly every time
pub struct DATA_master{
    pub DATA_player: player::TEMPLATE_player,
    pub DATA_world: world::TEMPLATE_world,
    pub DATA_debug: String,
    pub DATA_textItems: Vec<renderer::RENDER_textItem>,
    pub DATA_playerInput: logic::GAME_interactions,
    DATA_cache: HashMap<String, CACHE_TYPE>
}
impl DATA_master {

    /// # Push text for rendering
    /// 
    /// # DO NOT RELY ON THIS
    /// Will also be rewritten in favor of Window system
    pub fn DATA_pushTextItem(&mut self, INr_textItem: renderer::RENDER_textItem){
        self.DATA_textItems.push(INr_textItem)
    }

    pub fn DATA_pushDebugStr(&mut self, IN_string: String){
        self.DATA_debug.push_str(&format!("{}{NEW}", IN_string, NEW = system::SYS_NEWLINE))
    }

    /// # Get cache data
    /// Supply it with the index you stored the Cache at
    /// 
    /// If it doesn't find the cache it'll return `None`
    pub fn DATA_cacheData_GET(&self, IN_dataIndex: String) -> Option<&CACHE_TYPE>{
        match self.DATA_cache.get(&IN_dataIndex){
            None => return None,
            Some(cacheData) => return Some(cacheData)
        }
    }

    /// # Add cache data
    /// Supply it with index to store cache at and type of cache you want to store
    pub fn DATA_cacheData_ADD(&mut self, IN_dataIndex: String, IN_data: CACHE_TYPE){
        self.DATA_cache.insert(IN_dataIndex, IN_data);
    }

    /// # Free cache data
    /// Supply it with index of cache you don't need anymore
    pub fn DATA_cacheData_FREE(&mut self, IN_dataIndex: String){
        self.DATA_cache.remove(&IN_dataIndex);
    }
}

/// # Master subsystem struct
/// Allows for easy addition/replacement of subsystems
pub struct SUBSYSTEM_master{
    pub SUBSYSTEM_logic: logic::SYS_LOGIC,
    pub SUBSYSTEM_renderer: renderer::SYS_RENDERER,
    pub SUBSYSTEM_input: input::SYS_INPUT
}

/// # Cache type
/// Allows you to store a selected type of cache
/// 
/// Can be extended for any other cache type you want
pub enum CACHE_TYPE {
    CACHE_usize(usize),
    CACHE_u8(u8),
    CACHE_coords([usize; 2]),
    CACHE_interactCode(logic::GAME_interactions)
}