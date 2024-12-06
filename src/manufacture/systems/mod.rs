use super::*;

use components::*;
use resources::*;
use vars::*;
use types::*;

mod renderer;
pub use renderer::*;

mod tileStuff;
pub use tileStuff::*;

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
        
        for (ID, _) in VELCOMPS.proxyMap.iter(){
            let w_velComp = VELCOMPS.get(ID);
            let w_posComp = POSCOMPS.get_mut(ID);
            w_posComp.x += w_velComp.x;
        }
    }
}
pub struct sysData_Move<'a>{
    comp_Vel: ReadStorage<'a, comp_Vel>,
    comp_Pos: WriteStorage<'a, comp_Pos>
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
            IN_data.res_Input.res = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE);
        }

        if let Event::Key(KEY) = read().unwrap(){
            IN_data.res_Input.res = KEY
        }
    }
}
pub struct sysData_Input<'a>{
    res_Input: FetchMut<'a, res_PInput>
}
impl<'a> gmSystemData<'a> for sysData_Input<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_Input: IN_world.fetchResMut::<res_PInput>()
        }
    }
}

pub struct sys_PMove{}
impl<'a> gmSystem<'a> for sys_PMove{
    type sysData = sysData_PMove<'a>;

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_PMove"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        for (_, GMOBJID) in IN_data.res_PID.res.iter(){
            if !IN_data.comp_PController.get(GMOBJID).active{
                continue
            }
            let w_stepSize: isize = if IN_data.res_PInput.modifiers == KeyModifiers::SHIFT{
                    4
                }else{
                    1
                };
            let w_velComp = IN_data.comp_Vel.get_mut(GMOBJID);
            match IN_data.res_PInput.code{
                KeyCode::Up => {w_velComp.x = 0; w_velComp.y = w_stepSize}
                KeyCode::Down => {w_velComp.x = 0; w_velComp.y = -w_stepSize}
                KeyCode::Left => {w_velComp.x = -w_stepSize; w_velComp.y = 0}
                KeyCode::Right => {w_velComp.x = w_stepSize; w_velComp.y = 0}
                _ => {}
            }
        }
    }
}
pub struct sysData_PMove<'a>{
    res_PInput: Fetch<'a, res_PInput>,
    res_PID: Fetch<'a, res_PID>,
    comp_PController: ReadStorage<'a, comp_PController>,
    comp_Vel: WriteStorage<'a, comp_Vel>
}
impl<'a> gmSystemData<'a> for sysData_PMove<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_PInput: IN_world.fetchRes(),
            res_PID: IN_world.fetchRes(),
            comp_PController: IN_world.fetch(),
            comp_Vel: IN_world.fetchMut()
        }
    }
}