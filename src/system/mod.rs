use super::*;

pub mod vars;
pub mod types;
pub mod statics;
pub mod debug;
pub mod json;
pub mod input;

pub fn SYS_CHECK(){

    // Check tickrate
    assert!((vars::SYS::TICKRATE > 8) && (vars::SYS::TICKRATE < 32));

    // Check world size
    assert!(vars::WORLD::GENERAL::WORLD_X > 1);
    assert!(vars::WORLD::GENERAL::WORLD_Y > 1);

    // Check chunk size
    assert!(vars::WORLD::GENERAL::CHUNK_X > 1);
    assert!(vars::WORLD::GENERAL::CHUNK_Y > 1);

    // Check forest generation
    assert!(vars::WORLD::GENERATION::GEN_FOREST_ITERS.0 < vars::WORLD::GENERATION::GEN_FOREST_ITERS.1);
    assert!(vars::WORLD::GENERATION::GEN_FOREST_Q.0 < vars::WORLD::GENERATION::GEN_FOREST_Q.1);
    assert!(vars::WORLD::GENERATION::GEN_FOREST_SIZE.0 < vars::WORLD::GENERATION::GEN_FOREST_SIZE.1);

    // Check lake generation
    assert!(vars::WORLD::GENERATION::GEN_POND_ITERS.0 < vars::WORLD::GENERATION::GEN_POND_ITERS.1);
    assert!(vars::WORLD::GENERATION::GEN_POND_ITERS.0 < vars::WORLD::GENERATION::GEN_POND_ITERS.1);
    assert!(vars::WORLD::GENERATION::GEN_POND_ITERS.0 < vars::WORLD::GENERATION::GEN_POND_ITERS.1);

    // Check renderer stuff
    assert!(vars::RENDERER::RENDER_BUFFER_X > 0);
    assert!(vars::RENDERER::RENDER_BUFFER_Y > 0);
    assert!(vars::RENDERER::RENDER_CHUNKRAD > 2);
}