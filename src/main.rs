use crossterm::terminal::enable_raw_mode;

use std::{thread::sleep, time::{Duration, Instant}};

use manufacture::*;

// SYS_[...]        -- essentials
// TEMPLATE_[...]   -- templates for stuff
// GAME_[...]       -- actuall objects
// x_[...]          -- struct functions, x is first letter of struct name
// [...]_[...]      -- local variables, first [...] is function name in CAPS
// INx_[...]        -- input variables for functions, x is first letter of struct name

fn main() {
    enable_raw_mode().unwrap();
    let mut SYS_GAME_START = SYS_GAME::new();
    SYS_GAME_START.GAME_loop()
}

/// # Game struct
/// Contains the player and World and does all the logic
/// 
/// Renderer is a field so it can be replaced with a pixel one later on should I decide so
pub struct SYS_GAME {
    GAME_player: player::TEMPLATE_player,
    GAME_world: world::TEMPLATE_world,
    GAME_renderer: renderer::SYS_RENDERER,
    GAME_logic: logic::SYS_LOGIC,
    GAME_input: input::SYS_INPUT

}
impl SYS_GAME {

    /// # New game
    /// ## Disclaimer
    /// By default it sets the player as Player 1, change the number and `INp_color` if you wanna change it
    /// 
    /// Multiplayer will be added in the future
    pub fn new() -> Self{
        SYS_GAME { 
            GAME_player: player::TEMPLATE_player::new(1, None), 
            GAME_world: world::TEMPLATE_world::new(), 
            GAME_renderer: renderer::SYS_RENDERER::new(),
            GAME_logic: logic::SYS_LOGIC {},
            GAME_input: input::SYS_INPUT {}
        }
    }

    /// # Game loop
    /// Processes the whole game sequentially
    /// 
    /// Speed is dependent on `SYS_TICKRATE` value in `system.rs`
    /// 
    /// I do not recommend going above 32 ticks/s
    pub fn GAME_loop(&mut self) {
        self.GAME_world.w_generateRandom();
        loop {
            let loopStart: Instant = Instant::now();

            self.GAME_logic.GAME_interact(&mut self.GAME_world, &mut self.GAME_player, &mut self.GAME_renderer, self.GAME_input.SYS_HANDLER_input());

            self.GAME_player.p_updateChunkPos();

            self.GAME_renderer.r_pushDebugStr(&format!(
                "X: {}, Y: {}\nChunk X:{} Chunk Y:{}\n",
                self.GAME_player.p_x,
                self.GAME_player.p_y,
                self.GAME_player.p_chunkX,
                self.GAME_player.p_chunkY
            ));

            self.GAME_renderer.SYS_HANDLER_renderGame(&self.GAME_player, &self.GAME_world);

            let loop_elapsedTime: Duration = loopStart.elapsed();
            if loop_elapsedTime < system::SYS_TICKTIME {
                self.GAME_renderer.r_pushDebugStr(&format!(
                    "Too Fast! | {:?}\n Target speed: {:?}\n",
                    loop_elapsedTime, system::SYS_TICKTIME
                ));
                sleep(system::SYS_TICKTIME - loop_elapsedTime)
            } else {
                self.GAME_renderer.r_pushDebugStr(&format!("Too slow! | {:?}\n", loop_elapsedTime))
            }
        }
    }


}