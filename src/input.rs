use std::{process::exit, time::Duration};
use crossterm::event::*;

use crate::*;

pub struct SYS_INPUT{

}

impl SYS_INPUT {
    /// # Input handler
    /// # DO NOT RELY ON CURRENT VERSION OF THIS
    /// It will get updated with Window system and will read from a config file instead of single layout
    pub fn SYS_HANDLER_input(&self) -> logic::GAME_interactions{
        if poll(Duration::from_millis(25)).unwrap() {
            if let Event::Key(KeyEvent {code, modifiers: _, state: _, kind,}) = read().unwrap()
            {
                if kind != KeyEventKind::Press {
                    return logic::GAME_interactions::i_NULL;
                }
                match code {
                    KeyCode::Up => {
                        return logic::GAME_interactions::i_movPlayer(0);
                    }
                    KeyCode::Down => {
                        return logic::GAME_interactions::i_movPlayer(1);
                    }
                    KeyCode::Left => {
                        return logic::GAME_interactions::i_movPlayer(2);
                    }
                    KeyCode::Right => {
                        return logic::GAME_interactions::i_movPlayer(3);
                    }
                    KeyCode::Char('f') => return logic::GAME_interactions::i_printHello,
                    KeyCode::Char('g') => return logic::GAME_interactions::i_printDebug,
                    KeyCode::Char('h') => return logic::GAME_interactions::i_changeWorldTile,
                    KeyCode::Char('j') => return logic::GAME_interactions::i_clearWorld,
                    KeyCode::Esc => exit(0),
                    _ => {return logic::GAME_interactions::i_NULL}
                }
            }
        }
        //self.GAME_renderer.r_pushDebugStr("No input, skipping\n");
        return logic::GAME_interactions::i_NULL;
    }
}