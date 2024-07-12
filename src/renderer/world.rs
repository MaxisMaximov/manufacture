use super::*;

/// # Render the world
pub fn r_util_world() {
    let mut BUFFER_LOCK = buffer.lock().unwrap();
    let DATA_LOCK = statics::data.lock().unwrap();
    // First get vec of chunk references to not overload the system
    let r_workingChunkArray = DATA_LOCK.world.returnChunkArray(
        DATA_LOCK.player.chunk,
         vars::RENDERER::RENDER_CHUNKRAD,
    );

    // Calc border offset
    // Player offset in chunk + Chunk radius offset - radius
    let r_workingBorderOffset: types::vector2 = (
        // X
        (DATA_LOCK.player.loc.0 % vars::WORLD::GENERAL::CHUNK_X
            + vars::RENDERER::RENDER_CHUNKRAD * vars::WORLD::GENERAL::CHUNK_X)
            - vars::RENDERER::RENDER_WORLD_X,
        // Y
        (DATA_LOCK.player.loc.1 % vars::WORLD::GENERAL::CHUNK_Y
            + vars::RENDERER::RENDER_CHUNKRAD * vars::WORLD::GENERAL::CHUNK_Y)
            - vars::RENDERER::RENDER_WORLD_Y,
    );

    // Quickset X position
    let mut w_bufferX: usize = 2;

    for XPOS in 0..vars::RENDERER::RENDER_WORLDSIZE_X{

        // Quickset Y position
        let mut w_bufferY: usize = 2;

        // Just to not recalc every Y iter
        let idkfa_posX: usize = r_workingBorderOffset.0 + XPOS;

        for YPOS in 0..vars::RENDERER::RENDER_WORLDSIZE_Y{

            let idkfa_posY = r_workingBorderOffset.1 + YPOS;

            let w_cell = r_workingChunkArray[
                idkfa_posX/vars::WORLD::GENERAL::CHUNK_X + 
                idkfa_posY/vars::WORLD::GENERAL::CHUNK_Y * vars::RENDERER::RENDER_CHUNKRADSIZE]
                    [(idkfa_posX % vars::WORLD::GENERAL::CHUNK_X, idkfa_posY % vars::WORLD::GENERAL::CHUNK_Y)];

            // Finally set the buffer cell
            // Gotta find a cleaner way for this
            BUFFER_LOCK[(w_bufferX, w_bufferY)] = render_cell{char:w_cell.char, colors:w_cell.color};
            w_bufferY += 1
        }
        w_bufferX += 1
    }
}