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

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_Move"
    }

    fn execute(&mut self, IN_data: Self::sysData) {
        // Quick destructure
        let sysData_Move{
            comp_Vel: mut VELCOMPS,
            comp_Pos: mut POSCOMPS} = IN_data;
        
        for VELCOMP in VELCOMPS.inner.iter_mut(){

            // Don't bother if the entity can't move
            if VELCOMP.val.frozen{continue}

            // Every entity with Velocity MUST have a position component
            // Otherwise, what the peck are you doing
            let w_posComp = POSCOMPS.get_mut(&VELCOMP.id).expect("ERROR: Velocity component assigned to an entity without Position component");

            w_posComp.x += VELCOMP.val.x;
            w_posComp.y += VELCOMP.val.y;

            // "Simulate" drag
            // The syntax looks sooooo wrong, but I love it lol
            // Will just multiply by 0.75 once I'll switch to floats -- 0.75 cuz it's a nice number for Binary decimals -- or 0.875 as it's closer to the usual 0.9, but still nice for Binary decimals
            VELCOMP.val.x /= 2;
            VELCOMP.val.y /= 2;
            
        }
    }
}
pub struct sysData_Move<'a>{
    comp_Vel: WriteStorage<'a, comp_Vel>,
    comp_Pos: WriteStorage<'a, comp_Pos>
}
impl<'a> gmSystemData<'a> for sysData_Move<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_Vel: IN_world.fetchMut::<comp_Vel>(),
            comp_Pos: IN_world.fetchMut::<comp_Pos>(),
        }
    }
}

pub struct sys_Input{}
impl<'a> gmSystem<'a> for sys_Input{
    type sysData = sysData_Input<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_Input"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        if !poll(Duration::from_secs(0)).unwrap(){
            //I Impld Deref for both of the inner thingies and I still need a double deref here lol
            (**IN_data.res_Input) = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE);
            return;
        }

        if let Event::Key(KEY) = read().unwrap(){
            (**IN_data.res_Input) = KEY
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

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_PMove"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        for (PID, GMOBJID) in IN_data.res_PID.iter(){

            let w_PController = IN_data.comp_PController.get(GMOBJID).expect(&format!("ERROR: PID of player {PID} points to an object without a Player Controller"));
            if !w_PController.active{continue}

            let w_stepSize: isize = match IN_data.res_PInput.modifiers{
                KeyModifiers::SHIFT => {4}
                KeyModifiers::NONE => {1}
                _ => {0}
            };

            if let Some(w_velComp) = IN_data.comp_Vel.get_mut(GMOBJID){
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