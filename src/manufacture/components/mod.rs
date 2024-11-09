use super::*;

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
    pub x: usize,
    pub y: usize
}
impl gmComp for comp_Pos {
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_Pos"
    }
}

pub struct comp_Vel{
    pub x: usize,
    pub y: usize
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
        "comp_PController                   "
    }
}