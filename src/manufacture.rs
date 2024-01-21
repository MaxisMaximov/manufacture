use crossterm::{event::*, style::Color};
use std::{process::exit, thread::sleep, time::{Duration, Instant}};

use crate::index;
use crate::renderer;
use crate::system;

/// # Game struct
/// Contains the player and World and does all the logic
/// 
/// Renderer is a field so it can be replaced with a pixel one later on should I decide so
pub struct SYS_GAME {
    GAME_player: index::TEMPLATE_player,
    GAME_world: index::TEMPLATE_world,
    GAME_renderer: renderer::SYS_RENDERER

}
impl SYS_GAME {

    /// # New game
    /// ## Disclaimer
    /// By default it sets the player as Player 1, change the number and `INp_color` if you wanna change it
    /// 
    /// Multiplayer will be added in the future
    pub fn new() -> Self{
        SYS_GAME { 
            GAME_player: index::TEMPLATE_player::new(0, None), 
            GAME_world: index::TEMPLATE_world::new(), 
            GAME_renderer: renderer::SYS_RENDERER::new() }
    }

    /// # Game loop
    /// Processes the whole game sequentially
    /// 
    /// Speed is dependent on `SYS_TICKRATE` value in `system.rs`
    /// 
    /// I do not recommend going above 32 ticks/s
    pub fn GAME_loop(&mut self) {
        loop {
            let loopStart: Instant = Instant::now();

            self.SYS_HANDLER_input();

            self.GAME_renderer.r_pushDebugStr(&format!(
                "X: {}, Y: {}\nLocation in World array: {}\n",
                self.GAME_player.p_x,
                self.GAME_player.p_y,
                self.GAME_player.p_x + (self.GAME_player.p_y * system::SYS_GRID_Y as u16)
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

    /// # Input handler
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// It will get updated with Window system and will read from a config file instead of single layout
    fn SYS_HANDLER_input(&mut self) {
        if poll(Duration::from_millis(25)).unwrap() {
            if let Event::Key(KeyEvent {code, modifiers: _, state: _, kind,}) = read().unwrap()
            {
                if kind != KeyEventKind::Press {
                    return;
                }
                match code {
                    KeyCode::Up => {
                        self.GAME_player.p_move(0);
                    }
                    KeyCode::Down => {
                        self.GAME_player.p_move(1);
                    }
                    KeyCode::Left => {
                        self.GAME_player.p_move(2);
                    }
                    KeyCode::Right => {
                        self.GAME_player.p_move(3);
                    }
                    KeyCode::Char('f') => self.GAME_interact(index::GAME_interactions::i_printHello),
                    KeyCode::Char('g') => self.GAME_interact(index::GAME_interactions::i_printDebug),
                    KeyCode::Char('h') => self.GAME_interact(index::GAME_interactions::i_changeWorldTile),
                    KeyCode::Char('j') => self.GAME_interact(index::GAME_interactions::i_clearWorld),
                    KeyCode::Esc => exit(1),
                    _ => {}
                }
            }
        } else {
            self.GAME_renderer.r_pushDebugStr("No input, skipping\n");
        }
    }

    /// # Interaction manager
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// While I'm not sure how it will change exactly it does "global" interactions for now
    /// 
    /// Window system will have different way of managing those
    fn GAME_interact(&mut self, interactCode: index::GAME_interactions) {
        match interactCode {
            index::GAME_interactions::i_changeWorldTile => {
                self.GAME_world
                    .w_setCell(self.GAME_player.p_x, self.GAME_player.p_y, 'c', Color::Black, Color::Red)
            }
            index::GAME_interactions::i_printHello => self.GAME_renderer.r_pushText(index::RENDER_textItem {
                t_text: "Hello!\nHello!".to_string(),
                t_position: [0, 0],
                t_lifetime: 32,
            }),
            index::GAME_interactions::i_printDebug => self.GAME_renderer.r_pushText(index::RENDER_textItem {
                t_text: "DEBUG".to_string(),
                t_position: [32, 32],
                t_lifetime: 16,
            }),
            index::GAME_interactions::i_clearWorld => self.GAME_world.w_clearWorld(),
        }
    }


}
