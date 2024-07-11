use super::*;

/// # Render text
/// Renders text in `RENDER_text` vector
///
/// # DO NOT RELY ON THIS
/// It'll be most likely removed in favor of Window system
pub fn render_textBox() {
    let mut DATA_LOCK = statics::data.lock().unwrap();
    let mut BUFFER_LOCK = RENDER_mainBuffer.lock().unwrap();
    
    let mut RTEXT_charStartPosition: types::vector2;
    let mut RTEXT_charPosition: types::vector2;

    for RTEXT in DATA_LOCK.DATA_textItems.iter_mut() {

        RTEXT_charStartPosition = RTEXT.t_position.value();
        RTEXT_charPosition = RTEXT_charStartPosition;

        'RENDER_textBlocks: for RTEXT_char in RTEXT.t_string.clone().chars() {

            if RTEXT_char == '\n' {
                RTEXT_charStartPosition.1 += 1;
                RTEXT_charPosition = RTEXT_charStartPosition;
                continue;
            }

            BUFFER_LOCK[RTEXT_charPosition] = TEMPLATE_wrCell {
                c_char: RTEXT_char,
                c_colors: vars::MISC::COLORS::COLORS_DEF,
            };

            RTEXT_charPosition.0 += 1
        }

        RTEXT.TEXT_tickdown();
    }

    DATA_LOCK.DATA_textItemCleanup()
}

/// # Print debug and error stuff
pub fn render_debug() {
    // Lock and load
    let mut STDOUT_LOCK = stdout().lock();
    let mut DEBUG_LOCK = statics::debug.lock().unwrap();

    // Iterate debug stuff
    for DEBUGSTR in DEBUG_LOCK.inner.values_mut() {
        // Ignore these strings
        if DEBUGSTR.markForDel {
            continue;
        }

        let _ = write!(STDOUT_LOCK, "{} | {} <> {}\r\n",
            DEBUGSTR.class,
            render_util::misc::runtimeFmt(&DEBUGSTR.string, &DEBUGSTR.values),
            DEBUGSTR.lifetime
        );
        
        DEBUGSTR.tickdown()
    }

    // Print everything to screen
    let _ = STDOUT_LOCK.flush();

    // And clean up
    DEBUG_LOCK.cleanup();
}