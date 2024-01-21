use crossterm::{event::*, style::Color};
use std::{process::exit, thread::sleep, time::{Duration, Instant}};

use crate::index;
use crate::renderer;
use crate::system;

pub struct SYS_GAME {
    GAME_player: index::TEMPLATE_player,
    GAME_world: index::TEMPLATE_world,
    GAME_renderer: renderer::SYS_RENDERER

}
impl SYS_GAME {
    pub fn new() -> Self{
        SYS_GAME { 
            GAME_player: index::TEMPLATE_player::new(0, None), 
            GAME_world: index::TEMPLATE_world::new(), 
            GAME_renderer: renderer::SYS_RENDERER::new() }
    }

    pub fn GAME_loop(&mut self) {
        loop {
            let loopStart: Instant = Instant::now();

            self.SYS_HANDLER_input();

            self.GAME_renderer.SYS_HANDLER_renderGame(&self.GAME_player, &self.GAME_world);

            self.GAME_renderer.r_pushDebugStr(&format!(
                "X: {}, Y: {}\nLocation in World array: {}\n",
                self.GAME_player.p_x,
                self.GAME_player.p_y,
                self.GAME_player.p_x + (self.GAME_player.p_y * system::SYS_GRID_Y as u16)
            ));

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
    fn SYS_HANDLER_input(&mut self) {
        if poll(Duration::from_millis(25)).unwrap() {
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                state,
                kind,
            }) = read().unwrap()
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
