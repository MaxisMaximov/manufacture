use super::*;

pub struct sys_Renderer{}
impl<'a> gmSystem<'a> for sys_Renderer{
    type sysData = sysData_Renderer<'a>;

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_Renderer"
    }

    fn execute(&mut self, IN_data: Self::sysData) {
        todo!()
    }
}

pub struct sysData_Renderer<'a>{
    pub comp_Pos: Fetch<'a, comp_Pos>,
    pub comp_Sprite: Fetch<'a, comp_Sprite>
}
impl<'a> gmSystemData<'a> for sysData_Renderer<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_Pos: IN_world.fetch(),
            comp_Sprite: IN_world.fetch(),
        }
    }
}