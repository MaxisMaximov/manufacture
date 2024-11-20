use super::*;

pub struct sys_Renderer{}

pub struct sysData_Renderer<'a>{
    pub comp_Pos: Fetch<'a, comp_Pos>,
    pub comp_Sprite: Fetch<'a, comp_Sprite>
}