use clearscreen::clear;
use crossterm::style::{Color, Stylize};
use crossterm::{event::*, style, terminal::enable_raw_mode};
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};

// SYS - essentials
// TEMPLATE -- templates for stuff
// GAME -- actuall objects
// x_[..] -- struct functions, x is first letter of struct name
// [...]_[...] -- local variables, first [...] is function name in CAPS

const SYS_TICKRATE: u8 = 8;
const SYS_TICKTIME: Duration = Duration::from_millis(1000 / SYS_TICKRATE as u64);
const SYS_GRID_X: usize = 16;
const SYS_GRID_Y: usize = 16;
const SYS_REND_X: usize = 32;
const SYS_REND_Y: usize = 32;

struct TEMPLATE_player {
    p_x: u16,
    p_y: u16,
}
impl TEMPLATE_player {
    fn p_move(&mut self, dir: u8) {
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
                if self.p_y == (SYS_GRID_Y as u16 - 1) {
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
                if self.p_x == (SYS_GRID_X as u16 - 1) {
                    return;
                }
                self.p_x += 1
            }
            _ => {}
        }
    }
}

struct TEMPLATE_wrCell {
    c_char: char,
    c_colChr: style::Color,
    c_colBg: style::Color,
}
impl Default for TEMPLATE_wrCell {
    fn default() -> Self {
        TEMPLATE_wrCell {
            c_char: ' ',
            c_colChr: style::Color::White,
            c_colBg: style::Color::Black,
        }
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

struct TEMPLATE_world {
    cells: [TEMPLATE_wrCell; (SYS_GRID_X * SYS_GRID_Y)],
}
impl Default for TEMPLATE_world {
    fn default() -> Self {
        TEMPLATE_world {
            cells: [TEMPLATE_wrCell {
                ..Default::default()
            }; (SYS_GRID_X * SYS_GRID_Y)],
        }
    }
}
impl TEMPLATE_world {
    fn w_setCell(&mut self, x: u16, y: u16, character: char) {
        self.cells[(x + y * SYS_GRID_Y as u16) as usize].c_char = character;
    }
    fn w_clearWorld(&mut self) {
        self.cells.fill(TEMPLATE_wrCell {
            ..Default::default()
        })
    }
}

enum GAME_interactions {
    i_changeWorldTile,
    i_printHello,
    i_printDebug,
    i_clearWorld,
}

struct RENDER_textItem {
    text: String,
    position: [usize; 2],
    lifetime: u16,
}

struct SYS_GAME {
    GAME_player: TEMPLATE_player,
    GAME_world: TEMPLATE_world,
    RENDER_bufferGrid: [TEMPLATE_wrCell; SYS_REND_X * SYS_REND_Y],
    RENDER_text: Vec<RENDER_textItem>,
    RENDER_debug: String,
}

impl Default for SYS_GAME {
    fn default() -> Self {
        SYS_GAME {
            GAME_player: (TEMPLATE_player { p_x: 2, p_y: 2 }),
            GAME_world: (TEMPLATE_world {
                ..Default::default()
            }),
            RENDER_bufferGrid: [TEMPLATE_wrCell {
                ..Default::default()
            }; SYS_REND_X * SYS_REND_Y],
            RENDER_text: vec![RENDER_textItem {
                text: "Welcome!".to_string(),
                position: [0, 0],
                lifetime: 32,
            }],
            RENDER_debug: "".to_string(),
        }
    }
}
impl SYS_GAME {
    fn GAME_loop(&mut self) {
        loop {
            let loopStart: Instant = Instant::now();

            self.SYS_HANDLER_input();

            self.SYS_HANDLER_renderGame();

            self.RENDER_debug.push_str(&format!(
                "X: {}, Y: {}\nLocation in World array: {}\n",
                self.GAME_player.p_x,
                self.GAME_player.p_y,
                self.GAME_player.p_x + (self.GAME_player.p_y * SYS_GRID_Y as u16)
            ));

            println!("{}", self.RENDER_debug);

            let loop_elapsedTime: Duration = loopStart.elapsed();
            if loop_elapsedTime < SYS_TICKTIME {
                self.RENDER_debug.push_str(&format!(
                    "Too Fast! | {:?}\n Target speed: {:?}\n",
                    loop_elapsedTime, SYS_TICKTIME
                ));
                sleep(SYS_TICKTIME - loop_elapsedTime)
            } else {
                self.RENDER_debug
                    .push_str(&format!("Too slow! | {:?}\n", loop_elapsedTime))
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
                    KeyCode::Char('f') => self.GAME_interact(GAME_interactions::i_printHello),
                    KeyCode::Char('g') => self.GAME_interact(GAME_interactions::i_printDebug),
                    KeyCode::Char('h') => self.GAME_interact(GAME_interactions::i_changeWorldTile),
                    KeyCode::Char('j') => self.GAME_interact(GAME_interactions::i_clearWorld),
                    KeyCode::Esc => exit(1),
                    _ => {}
                }
            }
        } else {
            self.RENDER_debug.push_str("No input, skipping\n");
        }
    }

    fn GAME_interact(&mut self, interactCode: GAME_interactions) {
        match interactCode {
            GAME_interactions::i_changeWorldTile => {
                self.GAME_world
                    .w_setCell(self.GAME_player.p_x, self.GAME_player.p_y, 'c')
            }
            GAME_interactions::i_printHello => self.RENDER_text.push(RENDER_textItem {
                text: "Hello!\nHello!".to_string(),
                position: [0, 0],
                lifetime: 32,
            }),
            GAME_interactions::i_printDebug => self.RENDER_text.push(RENDER_textItem {
                text: "DEBUG".to_string(),
                position: [32, 32],
                lifetime: 16,
            }),
            GAME_interactions::i_clearWorld => self.GAME_world.w_clearWorld(),
        }
    }

    fn SYS_HANDLER_renderGame(&mut self) {
        let RENDER_start = Instant::now();

        // Reset screen and buffers
        clear();
        self.RENDER_bufferGrid.fill(TEMPLATE_wrCell {
            ..Default::default()
        });
        self.RENDER_debug.clear();

        self.RENDER_UTIL_border([1, 1], [SYS_GRID_X + 1, SYS_GRID_Y + 1]);

        self.RENDER_UTIL_border([1, 20], [5, 8]);

        self.RENDER_UTIL_world();

        self.RENDER_bufferGrid[self.RENDER_UTIL_calcPos(
            [self.GAME_player.p_x as usize, self.GAME_player.p_y as usize],
            [2, 2],
        )];

        self.RENDER_UTIL_text();

        // Convert buffer into string
        let mut RENDER_bufferstring = String::new();
        for YPOS in 0..SYS_REND_Y - 1 {
            for XPOS in 0..SYS_REND_X - 1 {
                let RENDER_cell = self.RENDER_bufferGrid[XPOS + YPOS * SYS_REND_Y];
                RENDER_bufferstring.push_str(
                    &RENDER_cell
                        .c_char
                        .with(RENDER_cell.c_colChr)
                        .on(RENDER_cell.c_colBg)
                        .to_string(),
                )
            }
            RENDER_bufferstring.push('\n')
        }
        println!("{}", RENDER_bufferstring);

        self.RENDER_debug.push_str(&format!(
            "Finished frame rendering in {:?}\n",
            RENDER_start.elapsed()
        ));
    }

    fn RENDER_UTIL_setBufferCell(
        &mut self,
        cPosition: usize,
        cChar: char,
        cColChr: Color,
        cColBg: Color,
    ) {
        self.RENDER_bufferGrid[cPosition] = TEMPLATE_wrCell {
            c_char: cChar,
            c_colChr: cColChr,
            c_colBg: cColBg,
        }
    }

    fn RENDER_UTIL_calcPos(&self, localPos: [usize; 2], offsetPos: [usize; 2]) -> usize {
        return ((localPos[0] + offsetPos[0]) + (localPos[1] + offsetPos[1]) * SYS_REND_Y);
    }

    fn RENDER_UTIL_border(&mut self, borderPos: [usize; 2], borderSizeInner: [usize; 2]) {
        // Corners first
        self.RENDER_bufferGrid[self.RENDER_UTIL_calcPos(borderPos, [0, 0])] = '╔';
        self.RENDER_bufferGrid
            [self.RENDER_UTIL_calcPos([borderPos[0] + borderSizeInner[0], borderPos[1]], [0, 0])] =
            '╗';
        self.RENDER_bufferGrid
            [self.RENDER_UTIL_calcPos([borderPos[0], borderPos[1] + borderSizeInner[1]], [0, 0])] =
            '╚';
        self.RENDER_bufferGrid[self.RENDER_UTIL_calcPos(
            [
                borderPos[0] + borderSizeInner[0],
                borderPos[1] + borderSizeInner[1],
            ],
            [0, 0],
        )] = '╝';

        // Top and bottom border
        for YPOS in [borderPos[1], borderPos[1] + borderSizeInner[1]] {
            for XPOS in borderPos[0] + 1..borderSizeInner[0] + 1 {
                self.RENDER_bufferGrid[self.RENDER_UTIL_calcPos([XPOS, YPOS], [0, 0])] = '=';
            }
        }
        // Left and right border
        for XPOS in [borderPos[0], borderPos[0] + borderSizeInner[0]] {
            for YPOS in borderPos[1] + 1..borderPos[1] + borderSizeInner[1] {
                self.RENDER_bufferGrid[self.RENDER_UTIL_calcPos([XPOS, YPOS], [0, 0])] = '‖';
            }
        }
    }

    fn RENDER_UTIL_text(&mut self) {
        for RTEXT_index in 0..self.RENDER_text.len() {
            let mut RTEXT_charStartIndex = self.RENDER_UTIL_calcPos(
                [
                    self.RENDER_text[RTEXT_index].position[0],
                    self.RENDER_text[RTEXT_index].position[1],
                ],
                [0, 0],
            );
            let mut RTEXT_charIndex = RTEXT_charStartIndex;
            'RENDER_textBlocks: for RTEXT_char in self.RENDER_text[RTEXT_index].text.chars() {
                if RTEXT_char == '\n' {
                    RTEXT_charIndex = RTEXT_charStartIndex + SYS_REND_Y;
                    continue;
                }
                if RTEXT_charIndex > 255 {
                    self.RENDER_debug.push_str(&format!(
                        "STRING ERROR: Out of Bounds\nString: --{}--\nLocation: X: {} Y: {}\n",
                        self.RENDER_text[RTEXT_index].text,
                        self.RENDER_text[RTEXT_index].position[0],
                        self.RENDER_text[RTEXT_index].position[1]
                    ));
                    break 'RENDER_textBlocks;
                }
                self.RENDER_bufferGrid[RTEXT_charIndex] = RTEXT_char;
                RTEXT_charIndex += 1
            }
            self.RENDER_text[RTEXT_index].lifetime -= 1;
        }

        self.RENDER_text.retain(|RTEXT| RTEXT.lifetime > 0)
    }

    fn RENDER_UTIL_world(&mut self) {
        let mut RWORLD_startIndex = self.RENDER_UTIL_calcPos([2, 2], [0, 0]);
        for WORLD_column in 0..SYS_GRID_Y {
            for WORLD_row in 0..SYS_GRID_X {
                self.RENDER_bufferGrid[RWORLD_startIndex + WORLD_row] =
                    self.GAME_world.cells[WORLD_row + WORLD_column * SYS_GRID_Y];
            }
            RWORLD_startIndex += SYS_REND_Y;
        }
    }
}

fn main() {
    enable_raw_mode().unwrap();
    let mut SYS_GAME_START: SYS_GAME = SYS_GAME {
        ..Default::default()
    };
    SYS_GAME_START.GAME_loop()
}
