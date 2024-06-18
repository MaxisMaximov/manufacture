use super::*;

/// # Render border
/// Lets you render a border at specific coords with specific size
pub fn main(borderPos: TYPE::vector2, borderSizeInner: TYPE::vector2) {
    
    let w_corners = (
        borderPos, // TL
        (borderPos.0 + borderSizeInner.0, borderPos.1), // TR
        (borderPos.0, borderPos.1 + borderSizeInner.1), // BL
        (borderPos.0 + borderSizeInner.0, borderPos.1 + borderSizeInner.1) // BR
    );
    
    render_util::util_lineDraw::main(w_corners.0, w_corners.1, '=', (Color::White, Color::Black));
    render_util::util_lineDraw::main(w_corners.2, w_corners.3, '=', (Color::White, Color::Black));
    render_util::util_lineDraw::main(w_corners.0, w_corners.2, '‖', (Color::White, Color::Black));
    render_util::util_lineDraw::main(w_corners.1, w_corners.3, '‖', (Color::White, Color::Black));
    
    let mut BUFFER_LOCK = self::RENDER_mainBuffer.lock().unwrap();
    // Corners at end cuz it's easier to set them
    'BORDER_CORNERS: {
        'TOP_LEFT: {
            BUFFER_LOCK[w_corners.0] = TEMPLATE_wrCell {
                c_char: '╔',
                c_colors: (Color::White, Color::Black),
            }
        }
        'TOP_RIGHT: {
            BUFFER_LOCK[w_corners.1] = TEMPLATE_wrCell {
                c_char: '╗',
                c_colors: (Color::White, Color::Black),
            }
        }
        'BOTTOM_LEFT: {
            BUFFER_LOCK[w_corners.2] = TEMPLATE_wrCell {
                c_char: '╚',
                c_colors: (Color::White, Color::Black),
            }
        }
        'BOTTOM_RIGHT: {
            BUFFER_LOCK[w_corners.3] = TEMPLATE_wrCell {
                c_char: '╝',
                c_colors: (Color::White, Color::Black),
            }
        }
    }
}