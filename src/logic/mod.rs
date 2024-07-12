use std::fmt;

use crate::*;

/// # Game logic
pub fn init(){
    let mut DEBUG_LOCK = statics::debug.lock().unwrap();
    'INIT_debugStr:{
        DEBUG_LOCK.inner.insert(
            ">LOGIC_interaction".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".LOGIC/#interaction",
                vars::MISC::PATHS::PATH_DEBUG,
                &[("{inter}", "".to_owned())], 
                255
            )
        );

        DEBUG_LOCK.inner.insert(
            ">SSINIT_logic".to_string(),
            debug::debug_item::new(
                debug::class::info,
                ".SYS/.SSINIT/#logic",
                vars::MISC::PATHS::PATH_DEBUG,
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
    // Lock and load
    let mut DATA_LOCK = statics::data.lock().unwrap();
    let mut WIDGET_LOCK = renderer::widgets::widgetsMap.lock().unwrap();

    // Holy hell this is long
    statics::debug
        .lock()
        .unwrap()
        .inner
        .get_mut(">LOGIC_interaction")
        .unwrap()
        .values[0].1 = format!("{}", DATA_LOCK.playerInput);

    match DATA_LOCK.playerInput {

        interactions::changeWorldTile => {
            let idkfa_pos: types::vector2 = DATA_LOCK.player.loc;
            let idkfa_colors: types::colorSet = (Color::Black, DATA_LOCK.player.color.1);
            DATA_LOCK.world[idkfa_pos] = data::world::world_cell{char: 'c', color: idkfa_colors};
        }

        interactions::printHello => 
            WIDGET_LOCK.textBoxes.push(
                renderer::widgets::textBox::new(
                    renderer::widgets::position::TL,
                    "Hello!\nHello!",
                    "",
                    32
                )
            ),

        interactions::printDebug =>
            WIDGET_LOCK.textBoxes.push(
                renderer::widgets::textBox::new(
                    renderer::widgets::position::right,
                    "DEBUG",
                    "",
                    16
                )
            ),

        interactions::clearWorld => DATA_LOCK.world.clearWorld(),

        interactions::movPlayer(dir) => {
            let idkfa_direction = dir;
            DATA_LOCK.player.walk(&idkfa_direction, vars::PLAYER::PLAYER_STEP_SIZE)}

        interactions::leapPlayer(dir) => {
            let idkfa_direction = dir;
            DATA_LOCK.player.walk(&idkfa_direction, vars::PLAYER::PLAYER_LEAP_SIZE)}

        interactions::NULL => {}
    }
}

/// # Interactions enum
/// # DON'T RELY ON THIS
/// It will be replaced with introduction of Window system
#[derive(Clone, Copy)]
pub enum interactions {
    NULL,
    movPlayer(playerDirections),
    leapPlayer(playerDirections),
    changeWorldTile,
    printHello,
    printDebug,
    clearWorld,
}
impl fmt::Display for interactions{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idkfa_pDir;
        let idkfa_fmt = match *self {
            Self::NULL => "NULL",
            Self::movPlayer(dir) => {idkfa_pDir = format!("movPlayer >> {}", dir); &idkfa_pDir},
            Self::leapPlayer(dir) => {idkfa_pDir = format!("leapPlayer >> {}", dir); &idkfa_pDir},
            Self::changeWorldTile => "changeWorldTile",
            Self::printHello => "printHello",
            Self::printDebug => "printDebug",
            Self::clearWorld => "clearWorld",
        };
        write!(f, "{}", idkfa_fmt)
    }
}

/// # Player direction enum
/// This exists solely for readbility
///
/// But also if I'd like to have more "advanced" movement
#[derive(Debug, Clone, Copy)]
pub enum playerDirections {
    up,
    down,
    left,
    right
}
impl fmt::Display for playerDirections{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idkfa_string = match *self {
            playerDirections::up => "Up",
            playerDirections::down => "Down",
            playerDirections::left => "Left",
            playerDirections::right => "Right"
        };
        write!(f, "{}", idkfa_string)
    }
}