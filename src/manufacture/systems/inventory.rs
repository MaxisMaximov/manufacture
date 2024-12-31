use super::*;

pub struct sys_InvOps{}
impl<'a> gmSystem<'a> for sys_InvOps{
    type sysData = sysData_InvOps<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_InvOps"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        for REMOVEOP in IN_data.event_ItemRem.iter(){
            if let Some(INVCOMP) = IN_data.comp_Inventory.get_mut(&REMOVEOP.target){
                // See if it find anything
                if let Some(INDEX) = INVCOMP.items.iter().position( |X| *X == REMOVEOP.item){
                    INVCOMP.items.remove(INDEX);
                }
            }
        }

        for ADDOP in IN_data.event_ItemAdd.iter(){
            if let Some(INVCOMP) = IN_data.comp_Inventory.get_mut(&ADDOP.target){
                if !INVCOMP.items.len() < INVCOMP.capacity{continue}
                INVCOMP.items.push(ADDOP.item);
            }
        }
    }
}
pub struct sysData_InvOps<'a>{
    comp_Inventory: WriteStorage<'a, comp_Inventory>,
    event_ItemAdd: EventReader<'a, event_InvOp_AddItem>,
    event_ItemRem: EventReader<'a, event_InvOp_RemoveItem>,
}
impl<'a> gmSystemData<'a> for sysData_InvOps<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_Inventory: IN_world.fetchMut(),
            event_ItemAdd: IN_world.fetchEventReader(),
            event_ItemRem: IN_world.fetchEventReader(),
        }
    }
}

pub struct sys_PInvOps{}
impl<'a> gmSystem<'a> for sys_PInvOps{
    type sysData = sysData_PInvOps<'a>;

    const sysDepends: &'static [&'static str] = &["sys_Input"];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_PInvOps"
    }

    fn execute(&mut self, mut IN_data: Self::sysData){
        match IN_data.res_PInput.code{
            KeyCode::Char('E') => {
                IN_data.event_InvOp_AddItem.push(
                    event_InvOp_AddItem{
                        item: 2,
                        target: *IN_data.res_PID.get(&1).expect(&format!("ERROR: PID of player 1 points to an object without an Inventory component"))});
            }
            KeyCode::Char('R') => {
                IN_data.event_InvOp_RemoveItem.push(
                    event_InvOp_RemoveItem{
                        item: 2,
                        target: *IN_data.res_PID.get(&1).expect(&format!("ERROR: PID of player 1 points to an object without an Inventory component"))});
            }
            _ => {}
        }
    }
}
pub struct sysData_PInvOps<'a>{
    res_PInput: Fetch<'a, res_PInput>,
    res_PID: Fetch<'a, res_PID>,
    event_InvOp_AddItem: EventWriter<'a, event_InvOp_AddItem>,
    event_InvOp_RemoveItem: EventWriter<'a, event_InvOp_RemoveItem>
}
impl<'a> gmSystemData<'a> for sysData_PInvOps<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_PInput: IN_world.fetchRes(),
            res_PID: IN_world.fetchRes(),
            event_InvOp_AddItem: IN_world.fetchEventWriter(),
            event_InvOp_RemoveItem: IN_world.fetchEventWriter(),
        }
    }
}