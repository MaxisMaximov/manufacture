use std::{process::exit, time::Duration};
use crossterm::event::*;

use crate::*;

pub fn init(){
    let mut DEBUG_LOCK = SYS_debug.lock().unwrap();

    'INIT_debugStr: {
        DEBUG_LOCK.DEBUG_items.insert(
            "#INPUT_keyType".to_string(),
            IDDQD_textItem::new(renderer::RENDER_position::None, ".DEBUG_input/#INPUT_keyType", "", 255)
        );

        DEBUG_LOCK.DEBUG_items.insert(
            "#INPUT_init".to_string(),
            IDDQD_textItem::new(renderer::RENDER_position::None, ".DEBUG_sys/.SYS_ssInit/#SSINIT_input", "", 40)
        );
    }
}
/// # Input handler
/// # DO NOT RELY ON CURRENT VERSION OF THIS
/// It will get updated with Window system and will read from a config file instead of single layout
pub fn main(){
    // Lock Data and Debug
    let mut DATA_LOCK = SYS_data.lock().unwrap();
    let mut DEBUG_LOCK = SYS_debug.lock().unwrap();

    // Check for input right away to not slow down the whole thing
    if poll(Duration::from_secs(0)).unwrap() {
        if let Event::Key(KeyEvent {code, modifiers, state: _, kind,}) = read().unwrap()
        {
            // Gotta skip the Repeat part cuz CMDs send Press and Repeat events at same time for some reason
            if kind != KeyEventKind::Press {
                DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_NULL;
                return;
            }
            DEBUG_LOCK.DEBUG_items.get_mut("#INPUT_keyType").unwrap().t_values = format!("{:?}", code);
            match code {
                KeyCode::Up => {
                    // Check if it should be a leap instead
                    if modifiers == KeyModifiers::SHIFT{
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_leapPlayer(player::GAME_playerDirections::DIR_up);
                        return;
                    }
                    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_up);
                }
                KeyCode::Down => {
                    // Check if it should be a leap instead
                    if modifiers == KeyModifiers::SHIFT{
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_leapPlayer(player::GAME_playerDirections::DIR_down);
                        return;
                    }
                    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_down);
                }
                KeyCode::Left => {
                    // Check if it should be a leap instead
                    if modifiers == KeyModifiers::SHIFT{
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_leapPlayer(player::GAME_playerDirections::DIR_left);
                        return;
                    }
                    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_left);
                }
                KeyCode::Right => {
                    // Check if it should be a leap instead
                    if modifiers == KeyModifiers::SHIFT{
                        DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_leapPlayer(player::GAME_playerDirections::DIR_right);
                        return;
                    }
                    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_movPlayer(player::GAME_playerDirections::DIR_right);
                }
                KeyCode::Char('f') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_printHello,
                KeyCode::Char('g') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_printDebug,
                KeyCode::Char('h') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_changeWorldTile,
                KeyCode::Char('j') => DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_clearWorld,
                KeyCode::Esc => {
                    let _ = execute!(stdout(), 
                        terminal::LeaveAlternateScreen, 
                        cursor::Show,
                        cursor::MoveTo(0, 0)
                    );
                    exit(0)
                },
                _ => {DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_NULL}
            }
            return;
        }
    }
    DEBUG_LOCK.DEBUG_items.get_mut("#INPUT_keyType").unwrap().t_values = "None".to_string();
    DATA_LOCK.DATA_playerInput = logic::GAME_interactions::i_NULL;
}