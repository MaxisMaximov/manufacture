use std::{process::exit, time::Duration};
use crossterm::event::*;

use crate::*;

pub struct SYS_INPUT{

}

impl SYS_INPUT {

    pub fn new() -> Self{
        SYS_INPUT {}
    }
    /// # Input handler
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// It will get updated with Window system and will read from a config file instead of single layout
    pub fn SYS_HANDLER_input(&self, SYS_data: &mut DATA_master){
        if poll(Duration::from_millis(25)).unwrap() {
            if let Event::Key(KeyEvent {code, modifiers: _, state: _, kind,}) = read().unwrap()
            {
                if kind != KeyEventKind::Press {
                    SYS_data.DATA_playerInput = logic::GAME_interactions::i_NULL;
                    return;
                }
                match code {
                    KeyCode::Up => {
                        SYS_data.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_up);
                    }
                    KeyCode::Down => {
                        SYS_data.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_down);
                    }
                    KeyCode::Left => {
                        SYS_data.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_left);
                    }
                    KeyCode::Right => {
                        SYS_data.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_right);
                    }
                    KeyCode::Char('f') => SYS_data.DATA_playerInput = logic::GAME_interactions::i_printHello,
                    KeyCode::Char('g') => SYS_data.DATA_playerInput = logic::GAME_interactions::i_printDebug,
                    KeyCode::Char('h') => SYS_data.DATA_playerInput = logic::GAME_interactions::i_changeWorldTile,
                    KeyCode::Char('j') => SYS_data.DATA_playerInput = logic::GAME_interactions::i_clearWorld,
                    KeyCode::Esc => exit(0),
                    _ => {SYS_data.DATA_playerInput = logic::GAME_interactions::i_NULL}
                }
                return;
            }
        }
        SYS_data.DATA_playerInput = logic::GAME_interactions::i_NULL;
        SYS_data.DATA_pushDebugStr("No input, skipping".to_string());
    }
}