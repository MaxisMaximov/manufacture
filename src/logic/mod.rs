use std::fmt;

use crate::*;

/// # Game logic
pub fn init(){
    let mut DEBUG_LOCK = statics::SYS_debug.lock().unwrap();
    'INIT_debugStr:{
        DEBUG_LOCK.DEBUG_items.insert(
            ">LOGIC_interaction".to_string(),
            debug::DEBUG_item::new(
                debug::DEBUG_class::info,
                ".LOGIC/#interaction",
                MISC::PATHS::PATH_DEBUG,
                &[("{inter}", "".to_owned())], 
                255
            )
        );

        DEBUG_LOCK.DEBUG_items.insert(
            ">SSINIT_logic".to_string(),
            debug::DEBUG_item::new(
                debug::DEBUG_class::info,
                ".SYS/.SSINIT/#logic",
                MISC::PATHS::PATH_DEBUG,
                &[], 
                40
            )
        );
    }
}

/// # Interaction manager
/// # DO NOT RELY ON CURRENT VERSION OF THIS
/// While I'm not sure how it will change exactly it does "global" interactions for now
/// 
/// Window system will have different way of managing those
pub fn main() {
    let mut DATA_LOCK = statics::SYS_data.lock().unwrap();

    statics::SYS_debug
        .lock()
        .unwrap()
        .DEBUG_items
        .get_mut(">LOGIC_interaction")
        .unwrap()
        .values[0].1 = format!("{}", DATA_LOCK.DATA_playerInput);

    match DATA_LOCK.DATA_playerInput {

        GAME_interactions::i_changeWorldTile => {
            let idkfa_pos: types::vector2 = DATA_LOCK.DATA_player.p_pos;
            let idkfa_colors: types::colorSet = (Color::Black, DATA_LOCK.DATA_player.p_color.1);
            DATA_LOCK.DATA_world[idkfa_pos] = data::world::TEMPLATE_wrCell{c_char: 'c', c_color: idkfa_colors};
        }

        GAME_interactions::i_printHello => 
            DATA_LOCK.DATA_textItems.push(
                IDDQD_textItem::new(renderer::RENDER_position::POS_TL, "Hello!\r\nHello!", "", 32)
            ),

        GAME_interactions::i_printDebug =>
            DATA_LOCK.DATA_textItems.push(
                IDDQD_textItem::new(renderer::RENDER_position::POS_right, "DEBUG", "", 16)
            ),

        GAME_interactions::i_clearWorld => DATA_LOCK.DATA_world.w_clearWorld(),

        GAME_interactions::i_movPlayer(dir) => {
            let idkfa_direction = dir;
            DATA_LOCK.DATA_player.p_move(&idkfa_direction, vars::PLAYER::PLAYER_STEP_SIZE)}

        GAME_interactions::i_leapPlayer(dir) => {
            let idkfa_direction = dir;
            DATA_LOCK.DATA_player.p_move(&idkfa_direction, vars::PLAYER::PLAYER_LEAP_SIZE)}

        GAME_interactions::i_NULL => {}
    }
}

/// # Interactions enum
/// # DON'T RELY ON THIS
/// It will be replaced with introduction of Window system
#[derive(Clone, Copy)]
pub enum GAME_interactions {
    i_NULL,
    i_movPlayer(GAME_playerDirections),
    i_leapPlayer(GAME_playerDirections),
    i_changeWorldTile,
    i_printHello,
    i_printDebug,
    i_clearWorld,
}
impl fmt::Display for GAME_interactions{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idkfa_pDir;
        let idkfa_fmt = match *self {
            Self::i_NULL => "NULL",
            Self::i_movPlayer(dir) => {idkfa_pDir = format!("movPlayer >> {}", dir); &idkfa_pDir},
            Self::i_leapPlayer(dir) => {idkfa_pDir = format!("leapPlayer >> {}", dir); &idkfa_pDir},
            Self::i_changeWorldTile => "changeWorldTile",
            Self::i_printHello => "printHello",
            Self::i_printDebug => "printDebug",
            Self::i_clearWorld => "clearWorld",
        };
        write!(f, "{}", idkfa_fmt)
    }
}

/// # Player direction enum
/// This exists solely for readbility
///
/// But also if I'd like to have more "advanced" movement
#[derive(Debug, Clone, Copy)]
pub enum GAME_playerDirections {
    DIR_up,
    DIR_down,
    DIR_left,
    DIR_right
}
impl fmt::Display for GAME_playerDirections{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idkfa_string = match *self {
            GAME_playerDirections::DIR_up => "Up",
            GAME_playerDirections::DIR_down => "Down",
            GAME_playerDirections::DIR_left => "Left",
            GAME_playerDirections::DIR_right => "Right"
        };
        write!(f, "{}", idkfa_string)
    }
}