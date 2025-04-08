use super::*;

use components::*;
use resources::*;
use constants::*;
use types::*;

mod renderer;
pub use renderer::*;

mod tile_stuff;
pub use tile_stuff::*;

pub struct SysMove{}
impl<'a> gmSystem<'a> for SysMove{
    type sysData = SysDataMove<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_Move"
    }

    fn execute(&mut self, IN_data: Self::sysData) {
        // Quick destructure
        let SysDataMove{
            comp_vel: mut VELCOMPS,
            comp_pos: mut POSCOMPS} = IN_data;
        
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
pub struct SysDataMove<'a>{
    comp_vel: WriteStorage<'a, CompVel>,
    comp_pos: WriteStorage<'a, CompPos>
}
impl<'a> gmSystemData<'a> for SysDataMove<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_vel: IN_world.fetchMut::<CompVel>(),
            comp_pos: IN_world.fetchMut::<CompPos>(),
        }
    }
}

pub struct SysInput{}
impl<'a> gmSystem<'a> for SysInput{
    type sysData = SysDataInput<'a>;

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
            (**IN_data.res_input) = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE);
            return;
        }

        if let Event::Key(KEY) = read().unwrap(){
            (**IN_data.res_input) = KEY
        }
    }
}
pub struct SysDataInput<'a>{
    res_input: FetchMut<'a, res_PInput>
}
impl<'a> gmSystemData<'a> for SysDataInput<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_input: IN_world.fetchResMut::<res_PInput>()
        }
    }
}

pub struct SysPmove{}
impl<'a> gmSystem<'a> for SysPmove{
    type sysData = SysDataPmove<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_PMove"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        for (PID, GMOBJID) in IN_data.res_pid.iter(){

            let w_PController = IN_data.comp_pcontroller.get(GMOBJID).expect(&format!("ERROR: PID of player {PID} points to an object without a Player Controller"));
            if !w_PController.active{continue}

            let w_stepSize: isize = match IN_data.res_pinput.modifiers{
                KeyModifiers::SHIFT => {4}
                KeyModifiers::NONE => {1}
                _ => {0}
            };

            if let Some(w_velComp) = IN_data.comp_vel.get_mut(GMOBJID){
                match IN_data.res_pinput.code{
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
pub struct SysDataPmove<'a>{
    res_pinput: Fetch<'a, res_PInput>,
    res_pid: Fetch<'a, res_PID>,
    comp_pcontroller: ReadStorage<'a, CompPcontroller>,
    comp_vel: WriteStorage<'a, CompVel>
}
impl<'a> gmSystemData<'a> for SysDataPmove<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_pinput: IN_world.fetchRes(),
            res_pid: IN_world.fetchRes(),
            comp_pcontroller: IN_world.fetch(),
            comp_vel: IN_world.fetchMut()
        }
    }
}