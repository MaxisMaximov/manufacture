#![allow(nonstandard_style)]
#![allow(unused_labels)]

use crossterm::terminal::enable_raw_mode;
use once_cell::sync::Lazy;

use std::{collections::HashMap, sync::Mutex, thread::sleep, time::{Duration, Instant}};

mod logic;
mod player;
mod world;
mod input;
mod renderer;
mod system;
mod jsonManager;

// This is a mess.
pub static SYS_jsonManager: Lazy<Mutex<jsonManager::SYS_jsonManager>> = Lazy::new(|| Mutex::new(jsonManager::SYS_jsonManager::new()));
pub static SYS_data: Lazy<Mutex<DATA_master>> = Lazy::new(|| Mutex::new(DATA_master::new(player::TEMPLATE_player::new(1, None))));
pub static SYS_debug: Lazy<Mutex<DEBUG_master>> = Lazy::new(|| Mutex::new(DEBUG_master::new()));

// START HERE
fn main() {

    SYS_debug.lock().unwrap().DATA_debugItems.insert(
        "#SSINIT_data".to_string(),
        IDDQD_textItem::newDebug(".DEBUG_sys/.SYS_ssInit/#SSINIT_data", "", 40)
    );

    SYS_debug.lock().unwrap().DATA_debugItems.insert(
        "#SYS_processTime".to_string(),
        IDDQD_textItem::newDebug(".DEBUG_sys/#SYS_processSpeed", "", 255)
    );

    // Switch to Raw Mode
    enable_raw_mode().unwrap();

    // Generate new world
    // Commented out cuz for whatever reason it gets stuck in loop
    // Will fix it with new world gen
    //SYS_data.lock().unwrap().DATA_world.w_generateRandom();

    // Initialize the subsystems
    let mut SYS_renderer = renderer::SYS_RENDERER::new();
    let mut SYS_logic = logic::SYS_LOGIC::new();
    let SYS_input = input::SYS_INPUT::new();

    // # THE GAME LOOP
    loop {

        // Start the timer
        let loopStart: Instant = Instant::now();

        // Set next Player input to process
        SYS_input.SYS_HANDLER_input();

        // Process the input
        SYS_logic.GAME_interact();

        // Render everything
        SYS_renderer.SYS_HANDLER_renderGame();

        // Log how long it took to process everything
        SYS_debug.lock().unwrap().DATA_debugItems.get_mut("#SYS_processTime").unwrap().t_values = format!("{:?}", loopStart.elapsed());
        SYS_renderer.SYS_HANDLER_renderDebugStrs();

        let loop_elapsedTime: Duration = loopStart.elapsed();
        if loop_elapsedTime < system::SYS_TICKTIME {
            sleep(system::SYS_TICKTIME - loop_elapsedTime)
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
    pub DATA_textItems: Vec<IDDQD_textItem>,
    pub DATA_playerInput: logic::GAME_interactions,
    DATA_cache: HashMap<String, CACHE_TYPE>
}
impl DATA_master {
    pub fn new(IN_player: player::TEMPLATE_player) -> Self{
        Self{
            DATA_player: IN_player,
            DATA_world: world::TEMPLATE_world::new(),
            DATA_textItems: Vec::new(),
            DATA_playerInput: logic::GAME_interactions::i_NULL,
            DATA_cache: HashMap::new()
        }
    }
    pub fn DATA_textItemCleanup(&mut self){
        self.DATA_textItems.retain(|x| x.t_string == "#MARK_FOR_DELETION")
    }
}

/// # Master Debug Struct
/// Holds the Debug info from subsystems
/// 
/// Reason I made this?  
/// So that Deadlocks don't happen with `SYS_data` because apparently it really likes to do that
pub struct DEBUG_master{
    pub DATA_debugItems: HashMap<String, IDDQD_textItem>,
    pub DATA_debugStrs: Vec<String>
}
impl DEBUG_master{
    pub fn new() -> Self{
        Self{
            DATA_debugItems: HashMap::new(),
            DATA_debugStrs: Vec::new()
        }
    }

    /// Clean up the hashmap
    /// A.k.a. get rid of `#MARK_FOR_DELETION` entries
    pub fn DEBUG_cleanup(&mut self){
        self.DATA_debugItems.retain(|_, v| v.t_string == "#MARK_FOR_DELETION")
    }
}

/// # Cache type
/// Allows you to store a selected type of cache
/// 
/// Can be extended for any other cache type you want
pub enum CACHE_TYPE {
    CACHE_usize(usize),
    CACHE_u8(u8),
    CACHE_coords(system::coords),
    CACHE_interactCode(logic::GAME_interactions)
}

/// # Text/Debug struct
/// 
/// TextItem and DebugItem struct in one
/// 
/// Use respective `new` functions for what you want
pub struct IDDQD_textItem{
    pub t_position: renderer::RENDER_position,
    pub t_string: String,
    pub t_values: String,
    pub t_lifetime: u16
}
impl IDDQD_textItem{

    /// # Create new DebugItem
    /// 
    /// The text for debug at the bottom of the rendering
    pub fn newDebug(IN_strID: &str, IN_values: &str, IN_lifetime: u16) -> Self{
        Self{
            t_position: renderer::RENDER_position::None,
            t_string: SYS_jsonManager.lock().unwrap().JSON_FETCH_debugStr(IN_strID), // Prefetch the debug string to give jsonManager some slack
            t_values: IN_values.to_string(),
            t_lifetime: IN_lifetime
        }
    }

    /// # Create new TextItem
    /// 
    /// The one used to place text somewhere in the game
    pub fn newText(IN_pos: renderer::RENDER_position, IN_text: &str, IN_lifetime: u16) -> Self{
        Self{
            t_position: IN_pos,
            t_string: IN_text.to_string(),
            t_values: "".to_string(),
            t_lifetime: IN_lifetime
        }
    }

    /// Tickdown lifetime  
    /// Just to make it clean
    pub fn TEXT_tickdown(&mut self){
        // If it's marked for del just ignore
        if self.t_string == "#MARK_FOR_DELETION"{
            return;
        }
        // If it's ""permament"" then don't do anything
        if self.t_lifetime == 255{
            return;
        }
        // If lifetime is 0, mark for deletion
        if self.t_lifetime == 0{
            self.TEXT_markForDel();
            return;
        }
        self.t_lifetime -= 1;
    }

    /// Mark the string for deletion
    pub fn TEXT_markForDel(&mut self){
        *self = Self{
            t_position: renderer::RENDER_position::None,
            t_string: "#MARK_FOR_DELETION".to_string(),
            t_values: "#MARK_FOR_DELETION".to_string(),
            t_lifetime: 0
        }
    }
}