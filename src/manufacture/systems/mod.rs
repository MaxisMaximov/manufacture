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

pub struct sys_Input{}
impl<'a> gmSystem<'a> for sys_Input{
    type sysData = sysData_Input<'a>;

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_Input"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        if !poll(Duration::from_secs(0)).unwrap(){
            IN_data.res_Input.inner.res = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE);
        }

        if let Event::Key(KEY) = read().unwrap(){
            IN_data.res_Input.inner.res = KEY
        }
    }
}
pub struct sysData_Input<'a>{
    res_Input: FetchResMut<'a, res_PInput>
}
impl<'a> gmSystemData<'a> for sysData_Input<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_Input: IN_world.fetchResMut::<res_PInput>()
        }
    }
}