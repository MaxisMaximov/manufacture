use crossterm::terminal::enable_raw_mode;

use std::{collections::HashMap, thread::sleep, time::{Duration, Instant}};

mod logic;
mod player;
mod world;
mod input;
mod renderer;
mod system;

// SYS_[...]        -- essentials
// TEMPLATE_[...]   -- templates for stuff
// GAME_[...]       -- actuall objects
// x_[...]          -- struct functions, x is first letter of struct name
// [...]_[...]      -- local variables, first [...] is function name in CAPS
// INx_[...]        -- input variables for functions, x is first letter of struct name

fn main() {
    enable_raw_mode().unwrap();
    let mut DATA_player = player::TEMPLATE_player::new(1, None);
    let mut DATA_world = world::TEMPLATE_world::new();
    DATA_world.w_generateRandom();
    let mut SYS_data = DATA_master{
        DATA_player: DATA_player,
        DATA_world: DATA_world,
        DATA_debug: String::new(),
        DATA_cache: HashMap::new()
    };


    let mut SYS_GAME_START = SYS_GAME{
        GAME_data: SYS_data,
        GAME_logic: logic::SYS_LOGIC {},
        GAME_input: input::SYS_INPUT {},
        GAME_renderer: renderer::SYS_RENDERER::new()
    };
    SYS_GAME_START.GAME_loop()
}

pub struct DATA_master{
    pub DATA_player: player::TEMPLATE_player,
    pub DATA_world: world::TEMPLATE_world,
    pub DATA_debug: String,
    DATA_cache: HashMap<String, CACHE_TYPE>
}
impl DATA_master {
    pub fn DATA_getCacheData(&self, IN_dataIndex: String) -> Option<&CACHE_TYPE>{
        match self.DATA_cache.get(&IN_dataIndex){
            None => return None,
            Some(cacheData) => return Some(cacheData)
        }
    }
    pub fn DATA_addCacheData(&mut self, IN_dataIndex: String, IN_data: CACHE_TYPE){
        self.DATA_cache.insert(IN_dataIndex, IN_data);
    }
    pub fn DATA_freeCacheData(&mut self, IN_dataIndex: String){
        self.DATA_cache.remove(&IN_dataIndex);
    }
}

/// # Game struct
/// Contains the player and World and does all the logic
/// 
/// Renderer is a field so it can be replaced with a pixel one later on should I decide so
struct SYS_GAME {
    GAME_data: DATA_master,
    GAME_renderer: renderer::SYS_RENDERER,
    GAME_logic: logic::SYS_LOGIC,
    GAME_input: input::SYS_INPUT

}
impl SYS_GAME {

    /// # Game loop
    /// Processes the whole game sequentially
    /// 
    /// Speed is dependent on `SYS_TICKRATE` value in `system.rs`
    /// 
    /// I do not recommend going above 32 ticks/s
    pub fn GAME_loop(&mut self) {
        loop {
            let loopStart: Instant = Instant::now();

            self.GAME_logic.GAME_interact(&mut self.GAME_data, &mut self.GAME_renderer, self.GAME_input.SYS_HANDLER_input());

            self.GAME_data.DATA_player.p_updateChunkPos();

            self.GAME_data.DATA_debug.push_str(&format!(
                "X: {}, Y: {}\nChunk X:{} Chunk Y:{}\n",
                self.GAME_data.DATA_player.p_x,
                self.GAME_data.DATA_player.p_y,
                self.GAME_data.DATA_player.p_chunkX,
                self.GAME_data.DATA_player.p_chunkY
            ));

            self.GAME_renderer.SYS_HANDLER_renderGame(&mut self.GAME_data);

            let loop_elapsedTime: Duration = loopStart.elapsed();
            if loop_elapsedTime < system::SYS_TICKTIME {
                self.GAME_data.DATA_debug.push_str(&format!(
                    "Too Fast! | {:?}\n Target speed: {:?}\n",
                    loop_elapsedTime, system::SYS_TICKTIME
                ));
                sleep(system::SYS_TICKTIME - loop_elapsedTime)
            } else {
                self.GAME_data.DATA_debug.push_str(&format!("Too slow! | {:?}\n", loop_elapsedTime))
            }
        }
    }
}

enum CACHE_TYPE {
    CACHE_usize(usize),
    CACHE_u8(u8),
    CACHE_doubleCoords([usize; 2])
}