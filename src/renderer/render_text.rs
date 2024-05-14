use crate::system;
use crate::SYS_data;
use crate::renderer::*;

/// # Render text
/// Renders text in `RENDER_text` vector
///
/// # DO NOT RELY ON THIS
/// It'll be most likely removed in favor of Window system
pub fn textBox() {
    let mut DATA_LOCK = SYS_data.lock().unwrap();
    let mut BUFFER_LOCK = RENDER_mainBuffer.lock().unwrap();

    let mut w_skipToNewline: bool = false;

    for RTEXT in DATA_LOCK.DATA_textItems.iter_mut() {

        let mut RTEXT_charStartPosition: system::coords = RTEXT.t_position.value();
        let mut RTEXT_charPosition: system::coords = RTEXT_charStartPosition;
        'RENDER_textBlocks: for RTEXT_char in RTEXT.t_string.clone().chars() {

            if RTEXT_char == '\r' {
                continue;
            }
            if RTEXT_char == '\n' {
                w_skipToNewline = false;
                RTEXT_charStartPosition.1 += 1;
                RTEXT_charPosition = RTEXT_charStartPosition;
                continue;
            }

            if w_skipToNewline{continue}

            // If X exceeds mark the skip
            if RTEXT_charPosition.0 >= system::SYS_REND_BUFFER_X{
                w_skipToNewline = true;
                continue;
            }

            // If Y exceeds there's no hope for the string
            if RTEXT_charPosition.1 >= system::SYS_REND_BUFFER_Y
            {
                break 'RENDER_textBlocks;
            }

            BUFFER_LOCK[RTEXT_charPosition] = TEMPLATE_wrCell {
                c_char: RTEXT_char,
                c_colors: system::SYS_DEFCOLORS,
            };
            RTEXT_charPosition.0 += 1
        }
        if RTEXT.t_lifetime == 255 {
            continue;
        }
        RTEXT.TEXT_tickdown();
    }

    DATA_LOCK
        .DATA_textItems
        .retain(|RTEXT| RTEXT.t_lifetime > 0)
}

pub fn debug() {
    let mut DEBUG_LOCK = SYS_debug.lock().unwrap();
    let mut STDOUT_LOCK = stdout().lock();
    for DEBUGSTR in DEBUG_LOCK.DATA_debugItems.values_mut() {
        // Ignore these
        if &DEBUGSTR.t_string == "#MARK_FOR_DELETION" {
            continue;
        }

        write!(
            STDOUT_LOCK,
            "{} {}\r\n",
            &DEBUGSTR.t_string.clone().with(system::SYS_DEBUGCOLORS.0),
            &DEBUGSTR.t_values.clone().with(system::SYS_DEBUGCOLORS.1)
        )
        .unwrap();

        DEBUGSTR.TEXT_tickdown()
    }
    STDOUT_LOCK.flush().unwrap();
}