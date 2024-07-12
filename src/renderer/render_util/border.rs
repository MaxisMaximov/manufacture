use super::*;

/// # Render border
/// Lets you render a border at specific coords with specific size
pub fn main(IN_borderPos: types::vector2, IN_borderSize: types::vector2) {
    
    let w_corners = (
        IN_borderPos, // TL
        (IN_borderPos.0 + IN_borderSize.0, IN_borderPos.1), // TR
        (IN_borderPos.0, IN_borderPos.1 + IN_borderSize.1), // BL
        (IN_borderPos.0 + IN_borderSize.0, IN_borderPos.1 + IN_borderSize.1) // BR
    );
    
    render_util::lineDraw::main(w_corners.0, w_corners.1, '=', vars::MISC::COLORS::COLORS_DEF);
    render_util::lineDraw::main(w_corners.2, w_corners.3, '=', vars::MISC::COLORS::COLORS_DEF);
    render_util::lineDraw::main(w_corners.0, w_corners.2, '‖', vars::MISC::COLORS::COLORS_DEF);
    render_util::lineDraw::main(w_corners.1, w_corners.3, '‖', vars::MISC::COLORS::COLORS_DEF);
    
    let mut BUFFER_LOCK = buffer.lock().unwrap();
    // Corners at end cuz it's easier to set them
    'BORDER_CORNERS: {
        'TOP_LEFT: {
            BUFFER_LOCK[w_corners.0] = render_cell {
                char: '╔',
                colors: vars::MISC::COLORS::COLORS_DEF,
            }
        }
        'TOP_RIGHT: {
            BUFFER_LOCK[w_corners.1] = render_cell {
                char: '╗',
                colors: vars::MISC::COLORS::COLORS_DEF,
            }
        }
        'BOTTOM_LEFT: {
            BUFFER_LOCK[w_corners.2] = render_cell {
                char: '╚',
                colors: vars::MISC::COLORS::COLORS_DEF,
            }
        }
        'BOTTOM_RIGHT: {
            BUFFER_LOCK[w_corners.3] = render_cell {
                char: '╝',
                colors: vars::MISC::COLORS::COLORS_DEF,
            }
        }
    }
}