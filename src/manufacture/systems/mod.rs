use super::*;

use components::*;
use resources::*;

pub struct sys_Move{}
impl<'a> gmSystem<'a> for sys_Move{
    type sysData = sysData_Move<'a>;

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_Move"
    }

    fn execute(&mut self, IN_data: Self::sysData) {
        let sysData_Move{comp_Vel: VELCOMPS, comp_Pos: mut POSCOMPS} = IN_data;
        
        for (ID, _) in VELCOMPS.inner.proxyMap.iter(){
            let w_velComp = VELCOMPS.inner.get(*ID);
            let w_posComp = POSCOMPS.inner.get_mut(*ID);
            w_posComp.x += w_velComp.x;
        }
    }
}
pub struct sysData_Move<'a>{
    comp_Vel: Fetch<'a, comp_Vel>,
    comp_Pos: FetchMut<'a, comp_Pos>
}
impl<'a> gmSystemData<'a> for sysData_Move<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_Vel: IN_world.fetch::<comp_Vel>(),
            comp_Pos: IN_world.fetchMut::<comp_Pos>(),
        }
    }
}