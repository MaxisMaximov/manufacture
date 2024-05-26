#![allow(nonstandard_style)]
#![allow(unused_labels)]
#![warn(unused_crate_dependencies)]

use crossterm::{
    cursor, execute, style::Stylize, terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use once_cell::sync::Lazy;

use core::fmt;
use std::{
    collections::HashMap,
    io::stdout,
    sync::Mutex,
    thread::sleep,
    time::{Duration, Instant},
};

mod input;
mod jsonManager;
mod logic;
mod player;
mod renderer;
mod system;
mod world;
mod errorManager;

pub use system::*;
pub use errorManager::*;

// This is a mess.
pub static SYS_data: Lazy<Mutex<DATA_master>> =
    Lazy::new(|| Mutex::new(DATA_master::new(player::TEMPLATE_player::new(1, None))));
pub static SYS_debug: Lazy<Mutex<DEBUG_master>> = Lazy::new(|| Mutex::new(DEBUG_master::new()));
pub static SYS_errorQueue: Mutex<Vec<SYS_ERROR>> = Mutex::new(Vec::new());

// START HERE
fn main() {
    SYS_debug.lock().unwrap().DATA_debugItems.insert(
        "#SSINIT_data".to_string(),
        IDDQD_textItem::new(renderer::RENDER_position::None, ".DEBUG_sys/.SYS_ssInit/#SSINIT_data", "", 40),
    );

    SYS_debug.lock().unwrap().DATA_debugItems.insert(
        "#SYS_processTime".to_string(),
        IDDQD_textItem::new(renderer::RENDER_position::None, ".DEBUG_sys/#SYS_processSpeed", "", 255),
    );

    // Switch to Raw Mode
    enable_raw_mode().unwrap();

    // Hide the cursor
    let _ = execute!(stdout(), EnterAlternateScreen, cursor::Hide,);

    // Generate new world
    // Commented out cuz for whatever reason it gets stuck in loop
    // Will fix it with new world gen
    //SYS_data.lock().unwrap().DATA_world.w_generateRandom();

    // "Initialize" the subsystems
    renderer::init();
    logic::init();
    input::init();
    jsonManager::init();
    errorManager::init();

    // # THE GAME LOOP
    loop {
        // Start the timer
        let loopStart: Instant = Instant::now();

        // Set next Player input to process
        input::main();

        // Process the game
        logic::main();

        // Render everything
        renderer::main();

        // Log how long it took to process everything

        let mut DEBUG_LOCK = SYS_debug.lock().unwrap();
        let idkfa_debugString = DEBUG_LOCK.DATA_debugItems.get_mut("#SYS_processTime");
        idkfa_debugString.unwrap()
            .t_values = format!("{:?}", loopStart.elapsed());

        let loop_elapsedTime: Duration = loopStart.elapsed();
        if loop_elapsedTime < system::SYS_TICKTIME {
            sleep(system::SYS_TICKTIME - loop_elapsedTime)
        }
    }
}

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

/// # Master Debug Struct
/// Holds the Debug info from subsystems
///
/// Reason I made this?  
/// So that Deadlocks don't happen with `SYS_data` because apparently it really likes to do that
pub struct DEBUG_master {
    pub DATA_debugItems: HashMap<String, IDDQD_textItem>,
    pub DATA_debugStrs: Vec<String>,
}
impl DEBUG_master {
    pub fn new() -> Self {
        Self {
            DATA_debugItems: HashMap::new(),
            DATA_debugStrs: Vec::new(),
        }
    }

    /// Clean up the hashmap
    /// A.k.a. get rid of `#MARK_FOR_DELETION` entries
    pub fn DEBUG_cleanup(&mut self) {
        self.DATA_debugItems
            .retain(|_, v| v.t_string != "#MARK_FOR_DELETION")
    }
}

/// # Cache type
/// Allows you to store a selected type of cache
///
/// Can be extended for any other cache type you want
pub enum CACHE_TYPE {
    CACHE_usize(usize),
    CACHE_u8(u8),
    CACHE_coords(vector2),
    CACHE_interactCode(logic::GAME_interactions),
}

/// # Text/Debug struct
///
/// TextItem and DebugItem struct in one
///
/// `t_position` is ignored for DebugItems
pub struct IDDQD_textItem {
    pub t_position: renderer::RENDER_position,
    pub t_string: String,
    pub t_values: String,
    pub t_lifetime: u16,
}
impl IDDQD_textItem {

    /// # Create new TextItem
    ///
    /// The one used to place text somewhere in the game
    pub fn new(IN_pos: renderer::RENDER_position, IN_text: &str, IN_values: &str, IN_lifetime: u16) -> Self {
        // Check if it's a debug string
        let idkfa_string = if IN_text.starts_with('.'){
            jsonManager::debugStr(IN_text).unwrap_or(IN_text.to_string())
        }
        else{
            IN_text.to_string()
        };
        Self {
            t_position: IN_pos,
            t_string: idkfa_string,
            t_values: IN_values.to_string(),
            t_lifetime: IN_lifetime,
        }
    }

    /// Tickdown lifetime  
    /// Just to make it clean
    pub fn TEXT_tickdown(&mut self) {
        // If it's marked for del just ignore
        if self.t_string == "#MARK_FOR_DELETION" {
            return;
        }
        // If it's ""permament"" then don't do anything
        if self.t_lifetime == 255 {
            return;
        }
        // If lifetime is 0, mark for deletion
        if self.t_lifetime == 0 {
            self.TEXT_markForDel();
            return;
        }
        self.t_lifetime -= 1;
    }

    /// Mark the string for deletion
    pub fn TEXT_markForDel(&mut self) {
        *self = Self {
            t_position: renderer::RENDER_position::None,
            t_string: "#MARK_FOR_DELETION".to_string(),
            t_values: "#MARK_FOR_DELETION".to_string(),
            t_lifetime: 0,
        }
    }
}
impl fmt::Display for IDDQD_textItem{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            &self.t_string.clone().with(MISC::COLORS::COLORS_DEF.0),
            &self.t_values.clone().with(MISC::COLORS::COLORS_DEF.1)
        )
    }
}
impl fmt::Debug for IDDQD_textItem{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            &self.t_string.clone().with(MISC::COLORS::COLORS_DEBUG.0),
            &self.t_values.clone().with(MISC::COLORS::COLORS_DEBUG.1)
        )
    }
}