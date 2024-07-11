use std::{process::exit, time::Duration};
use crossterm::event::*;

use super::*;

pub fn init(){
    let mut DEBUG_LOCK = statics::debug.lock().unwrap();

    'INIT_debugStr: {
        DEBUG_LOCK.DEBUG_items.insert(
            ">INPUT_keyType".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".INPUT/#keyType",
                MISC::PATHS::PATH_DEBUG,
                &[("{key}", "".to_owned())],
                255
            )
        );

        DEBUG_LOCK.DEBUG_items.insert(
            ">SYS_SSINIT_input".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".DEBUG_sys/.SYS_ssInit/#SSINIT_input",
                MISC::PATHS::PATH_DEBUG,
                &[],
                40
            )
        );
    }
}
/// # Input handler
/// # DO NOT RELY ON CURRENT VERSION OF THIS
/// It will get updated with Window system and will read from a config file instead of single layout
pub fn main(){
    // Lock Data and Debug
    let mut DATA_LOCK = statics::data.lock().unwrap();
    let mut DEBUG_LOCK = statics::debug.lock().unwrap();

    // Check for input right away to not slow down the whole thing
    if poll(Duration::from_secs(0)).unwrap() {
        if let Event::Key(KeyEvent {code, modifiers, state: _, kind,}) = read().unwrap()
        {
            // Gotta skip the Repeat part cuz CMDs send Press and Repeat events at same time for some reason
            if kind != KeyEventKind::Press {
                DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_NULL;
                return;
            }
            DEBUG_LOCK.DEBUG_items.get_mut(">INPUT_keyType").unwrap().values[0].1 = format!("{:?}", code);
            match code {
                KeyCode::Up => {
                    // Check if it should be a leap instead
                    if modifiers == KeyModifiers::SHIFT{
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_leapPlayer(logic::GAME_playerDirections::DIR_up);
                        return;
                    }
                    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(logic::GAME_playerDirections::DIR_up);
                }
                KeyCode::Down => {
                    // Check if it should be a leap instead
                    if modifiers == KeyModifiers::SHIFT{
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_leapPlayer(logic::GAME_playerDirections::DIR_down);
                        return;
                    }
                    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(logic::GAME_playerDirections::DIR_down);
                }
                KeyCode::Left => {
                    // Check if it should be a leap instead
                    if modifiers == KeyModifiers::SHIFT{
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_leapPlayer(logic::GAME_playerDirections::DIR_left);
                        return;
                    }
                    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(logic::GAME_playerDirections::DIR_left);
                }
                KeyCode::Right => {
                    // Check if it should be a leap instead
                    if modifiers == KeyModifiers::SHIFT{
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_leapPlayer(logic::GAME_playerDirections::DIR_right);
                        return;
                    }
                    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(logic::GAME_playerDirections::DIR_right);
                }
                KeyCode::Char('f') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_printHello,
                KeyCode::Char('g') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_printDebug,
                KeyCode::Char('h') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_changeWorldTile,
                KeyCode::Char('j') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_clearWorld,
                KeyCode::Esc => {
                    let _ = execute!(stdout(),
                        cursor::MoveTo(0, 0),
                        cursor::Show,
                        terminal::Clear(terminal::ClearType::All),
                        terminal::LeaveAlternateScreen 
                    );
                    exit(0)
                },
                _ => {DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_NULL}
            }
            return;
        }
    }
    DEBUG_LOCK.DEBUG_items.get_mut(">INPUT_keyType").unwrap().values[0].1 = "None".to_owned();
    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_NULL;
}