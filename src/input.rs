use std::{process::exit, time::Duration};
use crossterm::event::*;

use crate::*;

pub struct SYS_INPUT{

}

impl SYS_INPUT {

    pub fn new() -> Self{
        let mut DEBUG_LOCK = SYS_debug.lock().unwrap();

        'INIT_debugStr: {
            DEBUG_LOCK.DEBUG_debugStr_ADD(
                "#INPUT_keyType",
                ".DEBUG_input/#INPUT_whatKey",
                "",
                255
            );

            DEBUG_LOCK.DEBUG_debugStr_ADD(
                "#INPUT_init",
                ".DEBUG_sys/.SYS_ssInit/#SSINIT_input",
                "",
                40
            );
        }

        Self {}
    }
    /// # Input handler
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// It will get updated with Window system and will read from a config file instead of single layout
    pub fn SYS_HANDLER_input(&self){
        let mut DATA_LOCK = SYS_data.lock().unwrap();
        if poll(Duration::from_millis(1)).unwrap() {
            if let Event::Key(KeyEvent {code, modifiers: _, state: _, kind,}) = read().unwrap()
            {
                if kind != KeyEventKind::Press {
                    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_NULL;
                    return;
                }
                SYS_debug.lock().unwrap().DEBUG_debugStr_GET("#INPUT_keyType").unwrap().ds_updateValues(&format!("{:?}", code));
                match code {
                    KeyCode::Up => {
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_up);
                    }
                    KeyCode::Down => {
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_down);
                    }
                    KeyCode::Left => {
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_left);
                    }
                    KeyCode::Right => {
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_right);
                    }
                    KeyCode::Char('f') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_printHello,
                    KeyCode::Char('g') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_printDebug,
                    KeyCode::Char('h') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_changeWorldTile,
                    KeyCode::Char('j') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_clearWorld,
                    KeyCode::Esc => exit(0),
                    _ => {DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_NULL}
                }
                return;
            }
        }
        SYS_debug.lock().unwrap().DEBUG_debugStr_GET("#INPUT_keyType").unwrap().ds_updateValues("None");
        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_NULL;
    }
}