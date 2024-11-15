use super::*;

use components::*;
use resources::*;
use vars::*;

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
        for (_, GMOBJID) in IN_data.res_PID.inner.res.iter(){
            if !IN_data.comp_PController.inner.get(*GMOBJID).active{
                continue
            }
            let w_stepSize: isize = if IN_data.res_PInput.inner.res.modifiers == KeyModifiers::SHIFT{
                    4
                }else{
                    1
                };
            let w_velComp = IN_data.comp_Vel.inner.get_mut(*GMOBJID);
            match IN_data.res_PInput.inner.res.code{
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
    res_PInput: FetchRes<'a, res_PInput>,
    res_PID: FetchRes<'a, res_PID>,
    comp_PController: Fetch<'a, comp_PController>,
    comp_Vel: FetchMut<'a, comp_Vel>
}
impl<'a> gmSystemData<'a> for sysData_PMove<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_PInput: IN_world.fetchRes::<res_PInput>(),
            res_PID: IN_world.fetchRes::<res_PID>(),
            comp_PController: IN_world.fetch::<comp_PController>(),
            comp_Vel: IN_world.fetchMut::<comp_Vel>()
        }
    }
}

pub struct sys_PTileChange{}
impl<'a> gmSystem<'a> for sys_PTileChange{
    type sysData = sysData_PTileChange<'a>;

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_PTileChange"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        let w_PCoords = IN_data.comp_Pos.inner.get(*IN_data.res_PID.inner.res.get(&1).unwrap());
        let w_oldTile = IN_data.res_World.inner.getTileMut((w_PCoords.x, w_PCoords.y)).unwrap().mat;
        let mut w_tileChangeEvents = IN_data.res_Events.inner.getEventWriter::<event_TileChange>();

        match IN_data.res_PInput.inner.res.code{
            KeyCode::Char('f') => {
                w_tileChangeEvents.inner.push(event_TileChange{
                    coords: (w_PCoords.x, w_PCoords.y),
                    newTile: w_oldTile + 1,
                });
            }
            KeyCode::Char('F') => {
                w_tileChangeEvents.inner.push(event_TileChange{
                    coords: (w_PCoords.x, w_PCoords.y),
                    newTile: w_oldTile - 1,
                });
            }
            KeyCode::Char('g') => {
                w_tileChangeEvents.inner.push(event_TileChange{
                    coords: (w_PCoords.x, w_PCoords.y),
                    newTile: 0,
                });
            }
            _ => {return}
        }
    }
}
pub struct sysData_PTileChange<'a>{
    res_World: FetchResMut<'a, res_GridWorld>,
    res_PInput: FetchRes<'a, res_PInput>,
    res_PID: FetchRes<'a, res_PID>,
    res_Events: FetchRes<'a, res_Events>,
    comp_Pos: Fetch<'a, comp_Pos>,
}
impl<'a> gmSystemData<'a> for sysData_PTileChange<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_World: IN_world.fetchResMut::<res_GridWorld>(),
            res_PInput: IN_world.fetchRes::<res_PInput>(),
            res_PID: IN_world.fetchRes::<res_PID>(),
            res_Events: IN_world.fetchRes::<res_Events>(),
            comp_Pos: IN_world.fetch::<comp_Pos>()
        }
    }
}