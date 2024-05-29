use super::*;

/// # Render text
/// Renders text in `RENDER_text` vector
///
/// # DO NOT RELY ON THIS
/// It'll be most likely removed in favor of Window system
pub fn render_textBox() {
    let mut DATA_LOCK = SYS_data.lock().unwrap();
    let mut BUFFER_LOCK = RENDER_mainBuffer.lock().unwrap();

    for RTEXT in DATA_LOCK.DATA_textItems.iter_mut() {

        let mut RTEXT_charStartPosition: vector2 = RTEXT.t_position.value();
        let mut RTEXT_charPosition: vector2 = RTEXT_charStartPosition;
        'RENDER_textBlocks: for RTEXT_char in RTEXT.t_string.clone().chars() {

            if RTEXT_char == '\r' {
                continue;
            }
            if RTEXT_char == '\n' {
                RTEXT_charStartPosition.1 += 1;
                RTEXT_charPosition = RTEXT_charStartPosition;
                continue;
            }

            BUFFER_LOCK[RTEXT_charPosition] = TEMPLATE_wrCell {
                c_char: RTEXT_char,
                c_colors: MISC::COLORS::COLORS_DEF,
            };
            RTEXT_charPosition.0 += 1
        }
        if RTEXT.t_lifetime == 255 {
            continue;
        }
        RTEXT.TEXT_tickdown();
    }

    DATA_LOCK.DATA_textItemCleanup()
}

/// # Print debug and Error stuff
pub fn render_debug() {
    let mut DEBUG_LOCK = SYS_debug.lock().unwrap();
    let mut STDOUT_LOCK = stdout().lock();
    let mut ERROR_LOCK = SYS_errorQueue.lock().unwrap();

    for DEBUGSTR in DEBUG_LOCK.DATA_debugItems.values_mut() {
        // Ignore these strings
        if &DEBUGSTR.t_string == "#MARK_FOR_DELETION" {
            continue;
        }

        let _ = write!(STDOUT_LOCK, "{:?}\r\n", DEBUGSTR);

        DEBUGSTR.TEXT_tickdown()
    }

    for ERRORSTR in ERROR_LOCK.iter_mut(){
        // Ignore these errors
        if ERRORSTR.ERR_markForDel{
            continue;
        }

        let _ = write!(STDOUT_LOCK, "{:?}\r\n", ERRORSTR);

        ERRORSTR.ERR_tickdown()
    }

    DEBUG_LOCK.DEBUG_cleanup();
    ERROR_LOCK.retain(|x| x.ERR_lifetime > 0);

    STDOUT_LOCK.flush().unwrap();
}