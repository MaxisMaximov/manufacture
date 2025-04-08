use super::*;
use resources::UIElement;
use types::{Vector2, StyleSet, Node};

pub struct CompHp{
    pub val: u16
}
impl gmComp for CompHp{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "CompHp"
    }
}

pub struct CompPos{
    pub x: isize,
    pub y: isize
}
impl gmComp for CompPos {
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "CompPos"
    }
}

pub struct CompVel{
    pub x: isize,
    pub y: isize,
    pub frozen: bool
}
impl gmComp for CompVel {
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "CompVel"
    }
}

pub struct CompSprite{
    pub size_x: usize,
    pub size_y: usize,
    pub sprite: Vec<StyleSet>,
    pub z_depth: u16 // Again, 65k possible values. 255 probably too little, 4mil far too much. If you get ANYWHERE near even 1k go see a doctor
}
impl gmComp for CompSprite{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "CompSprite"
    }
}

pub struct CompPcontroller{
    pub active: bool
}
impl gmComp for CompPcontroller{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "CompPController"
    }
}

pub struct CompTileTerrainChunk{
    pub chunk: Vector2,
    pub fresh: bool
}
impl gmComp for CompTileTerrainChunk{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "CompTileTerrainChunk"
    }
}

pub struct CompViewportCamera{
    pub tracked_entity: gmID,
    pub offset: Vector2,
    pub active: bool
}
impl gmComp for CompViewportCamera{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "CompViewportCamera"
    }
}

pub struct CompGUI{
    pub position: Vector2,
    pub elements: Node<UIElement>
}
impl gmComp for CompGUI{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "CompGUI"
    }
}