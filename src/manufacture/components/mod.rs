use super::*;
use resources::UI_element;
use types::{Vector2, StyleSet, Node};

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
    pub sprite: Vec<StyleSet>
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

pub struct comp_ViewportCamera{
    pub trackedEntity: gmID,
    pub offset: Vector2,
    pub active: bool
}
impl gmComp for comp_ViewportCamera{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_ViewportCamera"
    }
}

pub struct comp_UIBox{
    pub position: Vector2,
    pub elements: Node<UI_element>
}
impl gmComp for comp_UIBox{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_UIBox"
    }
}