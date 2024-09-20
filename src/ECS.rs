use super::*;

pub trait gmCompEx{
    type COMP_STORAGE: gmStorageEx;
    fn COMP_ID() -> &'static str;
}
pub trait gmCompBox{}
impl<T> gmCompBox for T where T: gmCompEx{}

pub struct gmCompHealth{
    val: u16
}
impl gmCompEx for gmCompHealth{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompHealth"
    }
}

pub struct gmCompPosition{
    x: usize,
    y: usize
}
impl gmCompEx for gmCompPosition{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompPosition"
    }
}

pub struct gmCompVelocity{
    x: usize,
    y: usize
}
impl gmCompEx for gmCompVelocity{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompVelocity"
    }
}

pub struct gmCompTerrainChunk{
    cells: [types::styleSet; vars::WORLD::GENERAL::CHUNK_X * vars::WORLD::GENERAL::CHUNK_Y],
    needsUpdate: bool
}
impl gmCompEx for gmCompTerrainChunk{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompTerrainChunk"
    }
}

pub struct gmCompRender{
    size: types::vector2,
    sprite: &'static [types::styleSet],
    visible: bool
}
impl gmCompEx for gmCompRender{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompRender"
    }
}

pub struct gmCompPController{
    active: bool
}
impl gmCompEx for gmCompPController{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompPController"   
    }
}


pub struct gmObj{}