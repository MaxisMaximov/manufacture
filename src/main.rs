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

    SYS_debug.lock().unwrap().DEBUG_debugStr_ADD(
        "#SSINIT_data",
        ".DEBUG_sys/.SYS_ssInit/#SSINIT_data",
        "",
        40
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

    SYS_debug.lock().unwrap().DEBUG_debugStr_ADD(
        "#SYS_processTime",
        ".DEBUG_sys/#SYS_processSpeed",
        "",
        255
    );

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
        SYS_debug.lock().unwrap().DEBUG_debugStr_GET("#SYS_processTime").unwrap().ds_updateValues(&format!("{:?}", loopStart.elapsed()));
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
    pub DATA_textItems: Vec<RENDER_textItem>,
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

    /// # Push text for rendering
    /// 
    /// # DO NOT RELY ON THIS
    /// Will also be rewritten in favor of Window system
    pub fn DATA_pushTextItem(&mut self, INr_text: &str, INr_position: renderer::RENDER_position, INr_lifetime: u16){
        self.DATA_textItems.push(RENDER_textItem {t_text: INr_text.to_string(), t_position: INr_position, t_lifetime: INr_lifetime})
    }

    // region: CACHE

        /// # Get cache data
        /// Supply it with the index you stored the Cache at
        /// 
        /// If it doesn't find the cache it'll return `None`
        pub fn DATA_cacheData_GET(&self, IN_dataIndex: &str) -> Option<&CACHE_TYPE>{
            match self.DATA_cache.get(IN_dataIndex){
                None => return None,
                Some(cacheData) => return Some(cacheData)
            }
        }

        /// # Add cache data
        /// Supply it with index to store cache at and type of cache you want to store
        pub fn DATA_cacheData_ADD(&mut self, IN_dataIndex: &str, IN_data: CACHE_TYPE){
            self.DATA_cache.insert(IN_dataIndex.to_string(), IN_data);
        }
        
        /// # Free cache data
        /// Supply it with index of cache you don't need anymore
        pub fn DATA_cacheData_FREE(&mut self, IN_dataIndex: &str){
            self.DATA_cache.remove(IN_dataIndex);
        }
    // endregion: CACHE
}

/// # Master Debug Struct
/// Holds the Debug info from subsystems
/// 
/// Reason I made this?  
/// So that Deadlocks don't happen with `SYS_data` because apparently it really likes to do that
pub struct DEBUG_master{
    pub DATA_debugItems: HashMap<String, DEBUG_item>,
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
        self.DATA_debugItems.retain(|_, v| v.DEBUG_str == "#MARK_FOR_DELETION")
    }

    /// # Get debug string reference
    /// So that you can update it
    pub fn DEBUG_debugStr_GET(&mut self, IN_dataIndex: &str) -> Option<&mut DEBUG_item>{
        match self.DATA_debugItems.get_mut(IN_dataIndex){
            None => return None,
            Some(debugStr) => return Some(debugStr)
        }
    }

    /// # Add string for debug
    /// Supply it with index to store debug string at and the debug string values
    pub fn DEBUG_debugStr_ADD(&mut self, INd_dataIndex: &str, INd_ID: &str, INd_values: &str, INd_lifetime: u16){
        self.DATA_debugItems.insert(INd_dataIndex.to_string(), DEBUG_item::new(INd_ID, INd_values, INd_lifetime));
    }

    /// # Free debug string
    /// If you REALLY want to remove it
    pub fn DEBUG_debugStr_FREE(&mut self, INd_dataIndex: &str){
        self.DATA_debugItems.remove(INd_dataIndex);
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

/// # Debug string item
/// Holds ID of the debug string, the values for debug and lifetime
/// 
/// Set `lifetime` to 255 if it's something you wanna track at all times
pub struct DEBUG_item{
    pub DEBUG_str: String,
    pub DEBUG_values: String,
    pub DEBUG_lifetime: u16
}
impl DEBUG_item{
    pub fn new(INds_ID: &str, INds_values: &str, INds_lifetime: u16) -> Self{
        Self{
            DEBUG_str: SYS_jsonManager.lock().unwrap().JSON_FETCH_debugStr(INds_ID), // Autoconvert the ID into the String to not read from .json a sh#t ton
            DEBUG_values: INds_values.to_string(),
            DEBUG_lifetime: INds_lifetime
        }
    }

    /// # Tickdown lifetime
    /// Just to make it bit cleaner
    pub fn ds_tickdown(&mut self){
        // If it's ""permament"" then don't do anything
        if self.DEBUG_lifetime == 255{
            return;
        }
        // If lifetime is 0, mark for deletion
        if self.DEBUG_lifetime == 0{
            self.ds_markForDel();
            return;
        }
        self.DEBUG_lifetime -= 1;
    }

    /// # Update debug string values
    /// No it will not format the new values for you
    pub fn ds_updateValues(&mut self, INds_upVals: &str){
        self.DEBUG_values = INds_upVals.to_string()
    }

    /// # Mark debug string for deletion
    /// If you want to force it to be deleted
    pub fn ds_markForDel(&mut self){
        *self = Self {DEBUG_str: "#MARK_FOR_DELETION".to_string(), DEBUG_values: "NULL".to_string(), DEBUG_lifetime: 0}
    }
}

/// # "Textbox" struct
/// Lets you paste a text somewhere in the game screen
/// 
/// # DO NOT RELY ON THIS
/// It'll be replaced in favor of Window system
/// 
/// # Warning
/// The Renderer doesn't check if the text overflows the X position yet, only if it's outside the buffer
/// 
/// So be careful where and what you write
pub struct RENDER_textItem{
    pub t_position: renderer::RENDER_position,
    pub t_text: String,
    pub t_lifetime: u16
}
impl RENDER_textItem{
    pub fn new(IN_pos: renderer::RENDER_position, IN_text: &str, IN_lifetime: u16) -> Self{
        Self {
            t_position: IN_pos,
            t_text: IN_text.to_string(),
            t_lifetime: IN_lifetime
        }
    }
    /// # Tickdown lifetime
    pub fn TEXT_tickdown(&mut self){
        // If it's ""permament"" then don't do anything
        if self.t_lifetime == 255 {
            return;
        }
        self.t_lifetime -= 1
    }
}