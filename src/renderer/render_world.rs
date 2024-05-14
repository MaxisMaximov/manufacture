use crate::system;
use crate::SYS_data;
use crate::renderer::*;

/// # Render the world
pub fn r_util_world() {
    let mut BUFFER_LOCK = RENDER_mainBuffer.lock().unwrap();
    let DATA_LOCK = SYS_data.lock().unwrap();
    // First get vec of chunk references to not overload the system
    let r_workingChunkArray = DATA_LOCK.DATA_world.w_returnChunkArray(
        DATA_LOCK.DATA_player.p_chunk,
         system::SYS_REND_CHUNKRAD,
    );

    // Calc border offset
    // Player offset in chunk + Chunk radius offset - radius
    let r_workingBorderOffset: system::coords = (
        // X
        (DATA_LOCK.DATA_player.p_pos.0 % system::SYS_CHUNK_X
            + system::SYS_REND_CHUNKRAD * system::SYS_CHUNK_X)
            - system::SYS_REND_WORLD_X,
        // Y
        (DATA_LOCK.DATA_player.p_pos.1 % system::SYS_CHUNK_Y
            + system::SYS_REND_CHUNKRAD * system::SYS_CHUNK_Y)
            - system::SYS_REND_WORLD_Y,
    );

    // Quickset X position
    let mut w_bufferX: usize = 2;

    for XPOS in 0..system::SYS_REND_WORLDSIZE_X{

        // Quickset Y position
        let mut w_bufferY: usize = 2;

        // Just to not recalc every Y iter
        let idkfa_posX: usize = r_workingBorderOffset.0 + XPOS;

        for YPOS in 0..system::SYS_REND_WORLDSIZE_Y{

            let idkfa_posY = r_workingBorderOffset.1 + YPOS;

            let w_cell = r_workingChunkArray[
                idkfa_posX/system::SYS_CHUNK_X + 
                idkfa_posY/system::SYS_CHUNK_Y * system::SYS_REND_CHUNKRADSIZE]
                    [(idkfa_posX % system::SYS_CHUNK_X, idkfa_posY % system::SYS_CHUNK_Y)];

            // Finally set the buffer cell
            // Gotta find a cleaner way for this
            BUFFER_LOCK[(w_bufferX, w_bufferY)] = TEMPLATE_wrCell{c_char:w_cell.c_char, c_colors:w_cell.c_color};
            w_bufferY += 1
        }
        w_bufferX += 1
    }
}