use super::*;
use vars::Vector2;

pub struct comp_HP{
    pub val: u16
}
impl gmComp for comp_HP{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_HP"
    }
}

pub struct comp_Pos{
    pub x: isize,
    pub y: isize
}
impl gmComp for comp_Pos {
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_Pos"
    }
}

pub struct comp_Vel{
    pub x: isize,
    pub y: isize
}
impl gmComp for comp_Vel {
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_Vel"
    }
}

pub struct comp_Sprite{
    pub sizeX: usize,
    pub sizeY: usize,
    pub sprite: Vec<crate::types::styleSet>
}
impl gmComp for comp_Sprite{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_Sprite"
    }
}

pub struct comp_PController{
    pub active: bool
}
impl gmComp for comp_PController{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_PController"
    }
}

pub struct comp_TileTerrainChunk{
    pub chunk: Vector2
}
impl gmComp for comp_TileTerrainChunk{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_TileTerrainChunk"
    }
}