use crossterm::style::Color;

use crate::system;

// region: Structs

/// # Player struct
/// Use 1-4 in `fp_playerNum` when initializing to change the color
/// 
/// Available colors:
/// * 1 - Cyan
/// * 2 - Green
/// * 3 - Yellow
/// * 4 - Orange
/// 
/// # Custom colors
/// To instead use custom colors set `fp_playerNum` to 0 and `fp_color` to [`Color::Rgb`]
pub struct TEMPLATE_player {
    pub p_x: u16,
    pub p_y: u16,
    pub p_colorChar: Color,
    pub p_colorBg: Color,
}
impl TEMPLATE_player {
    pub fn new(INp_playerNum: usize, INp_color: Option<Color>) -> Self{
        let Fp_playerColor: Color = if INp_playerNum == 0{
            INp_color.unwrap()
        }
        else {
            GAME_playerColors[INp_playerNum]
        };
        TEMPLATE_player { p_x: 0, p_y: 0, p_colorChar: Color::White, p_colorBg: Fp_playerColor }
    }
    pub fn p_move(&mut self, dir: u8) {
        match dir {
            0 => {
                // Up
                if self.p_y == 0 {
                    return;
                }
                self.p_y -= 1
            }
            1 => {
                //Down
                if self.p_y == (system::SYS_GRID_Y as u16 - 1) {
                    return;
                }
                self.p_y += 1
            }
            2 => {
                //Left
                if self.p_x == 0 {
                    return;
                }
                self.p_x -= 1
            }
            3 => {
                //Right
                if self.p_x == (system::SYS_GRID_X as u16 - 1) {
                    return;
                }
                self.p_x += 1
            }
            _ => {}
        }
    }
}

/// # World/Render Buffer Cell
/// 
/// Values:
/// 
/// * Character
/// * Color for character
/// * Color for background
pub struct TEMPLATE_wrCell {
    pub c_char: char,
    pub c_colChr: Color,
    pub c_colBg: Color,
}
impl TEMPLATE_wrCell{
    pub fn new() -> Self{
        TEMPLATE_wrCell { c_char: ' ', c_colChr: Color::White, c_colBg: Color::Black }
    }
}
impl Copy for TEMPLATE_wrCell {}
impl Clone for TEMPLATE_wrCell {
    fn clone(&self) -> Self {
        TEMPLATE_wrCell {
            c_char: self.c_char,
            c_colChr: self.c_colChr,
            c_colBg: self.c_colBg,
        }
    }
}

/// # "Textbox" struct
/// Lets you paste a text somewhere in the game screen
/// 
/// # Warning
/// The Renderer doesn't check if the text overflows the X position yet, only if it's outside the buffer
/// 
/// So be careful where and what you write
pub struct RENDER_textItem{
    pub t_position: [usize; 2],
    pub t_text: String,
    pub t_lifetime: u16
}


/// # Game world
/// For now relies on `SYS_GRID` values in `system.rs`
/// 
/// TODO: Make it handle buildings and chunks
/// 
/// `w_clearWorld` function is for debug purposes for now
pub struct TEMPLATE_world {
    pub cells: [TEMPLATE_wrCell; (system::SYS_GRID_X * system::SYS_GRID_Y)],
}
impl TEMPLATE_world {
    pub fn new() -> Self{
        TEMPLATE_world { 
            cells: [TEMPLATE_wrCell::new(); system::SYS_GRID_X*system::SYS_GRID_Y]
         }
    }
    pub fn w_setCell(&mut self, x: u16, y: u16, character: char, colorChar: Color, colorBg: Color) {
        self.cells[(x + y * system::SYS_GRID_X as u16) as usize] = TEMPLATE_wrCell{c_char: character, c_colChr: colorChar, c_colBg: colorBg};
    }
    pub fn w_clearWorld(&mut self) {
        self.cells.fill(TEMPLATE_wrCell::new())
    }
}

// endregion: Structs

// region: Enums

/// # Interactions enum
/// # DON'T RELY ON THIS
/// It will be replaced with introduction of Window system
pub enum GAME_interactions {
    i_changeWorldTile,
    i_printHello,
    i_printDebug,
    i_clearWorld,
}

/// # Player color "enum"
/// ## Disclaimer:
/// Is only for Player 1-4 colors
pub const GAME_playerColors: [Color;4] = [
    Color::Cyan,
    Color::Green,
    Color::Yellow,
    Color::Rgb {r: 255, g: 153, b: 0}
];

// endregion: Enums