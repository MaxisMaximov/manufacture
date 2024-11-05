use super::*;

pub struct comp_HP{
    val: u16
}
impl gmComp for comp_HP{
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_HP"
    }
}

pub struct comp_Pos{
    x: usize,
    y: usize
}
impl gmComp for comp_Pos {
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_Pos"
    }
}

pub struct comp_Vel{
    x: usize,
    y: usize
}
impl gmComp for comp_Vel {
    type COMP_STORAGE = denseVecStorage<Self>;

    fn COMP_ID() -> &'static str {
        "comp_Vel"
    }
}