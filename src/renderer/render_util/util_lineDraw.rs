use super::*;

pub fn main(
    IN_pos_A: TYPE::vector2,
    IN_pos_B: TYPE::vector2,
    IN_char: char,
    IN_colors: TYPE::colorSet,
) {
    let mut BUFFER_LOCK = self::RENDER_mainBuffer.lock().unwrap();

    // Init start values
    let w_startPos: TYPE::vector2;
    let w_endPos: TYPE::vector2;

    // Calc delta distance between points
    let w_deltaX = IN_pos_A.0.abs_diff(IN_pos_B.0);
    let w_deltaY = IN_pos_A.1.abs_diff(IN_pos_B.1);

    // Check which is the main axis
    if w_deltaX >= w_deltaY{ // X Axis
        
        // Check and set position
        'CHECK_pos:{
            if IN_pos_A.0 < IN_pos_B.0{
                w_startPos = IN_pos_A;
                w_endPos = IN_pos_B
            }
            else{
                w_startPos = IN_pos_B;
                w_endPos = IN_pos_A
            }
        }
        
        let mut w_curY = w_startPos.1; // Set subaxis position
        let w_sign = if w_startPos.1 < w_endPos.1{false} else{true}; // Check what way the line is going, set sign if needed

        // Iterate
        for XPOS in w_startPos.0..=w_endPos.0{
            BUFFER_LOCK[(XPOS, w_curY)] = TEMPLATE_wrCell{ c_char: IN_char, c_colors: IN_colors };

            // Idk who made this equation but why does it work
            // SubaxisDelta * 2 > SuperaxisDelta
            if w_curY.abs_diff(w_endPos.1)*2 > XPOS.abs_diff(w_endPos.1){
                // If sign is enabled that means it goes down
                if w_sign{w_curY -= 1}
                else{w_curY += 1}
            }
        }
    }
    else{ // Y Axis

        // Check  and set position
        'CHECK_pos:{
            if IN_pos_A.1 < IN_pos_B.1{
                w_startPos = IN_pos_A;
                w_endPos = IN_pos_B
            }
            else{
                w_startPos = IN_pos_B;
                w_endPos = IN_pos_A
            }
        }

        let mut w_curX = w_startPos.0; // Set subaxis position
        let w_sign = if w_startPos.0 < w_endPos.0{false} else{true}; // Check what way the line is going, set sign if needed

        // Iterate
        for YPOS in w_startPos.1..=w_endPos.1{
            BUFFER_LOCK[(w_curX, YPOS)] = TEMPLATE_wrCell{ c_char: IN_char, c_colors: IN_colors };

            // Idk who made this equation but why does it work
            // SubaxisDelta * 2 > SuperaxisDelta
            if w_curX.abs_diff(w_endPos.0)*2 > YPOS.abs_diff(w_endPos.1){
                // If sign is enabled that means it goes left
                if w_sign {w_curX -= 1}
                else{w_curX += 1}
            }
        }
    }
}