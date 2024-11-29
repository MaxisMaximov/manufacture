use super::*;
use systems::*;
use resources::*;
use components::*;

pub fn init(IN_world: &mut gmWorld, IN_dispatch: &mut gmDispatcher){
    // Register Systems
    IN_dispatch.addSys::<sys_Input>(&[]);

    IN_dispatch.addSys::<sys_PMove>(&[]);

    IN_dispatch.addSys::<sys_Move>(&[]);

    IN_dispatch.addSys::<sys_PTileChange>(&[]);

    IN_dispatch.addSys::<sys_TileChunkUpdate>(&[]);

    IN_dispatch.addSys::<sys_TileChunkSpriteUpdate>(&[]);

    IN_dispatch.addSys::<sys_Renderer>(&[]);

    // Register Components
    IN_world.registerComp::<comp_HP>();

    IN_world.registerComp::<comp_PController>();

    IN_world.registerComp::<comp_Pos>();
    
    IN_world.registerComp::<comp_Vel>();

    IN_world.registerComp::<comp_Sprite>();

    IN_world.registerComp::<comp_TileTerrainChunk>();

    IN_world.registerComp::<comp_UIBox>();
    
    IN_world.registerComp::<comp_ViewportCamera>();

    // Register Resources
    IN_world.registerRes::<res_DeltaT>();

    IN_world.registerRes::<res_Events>();

    IN_world.registerRes::<res_GridWorld>();

    IN_world.registerRes::<res_PID>();

    IN_world.registerRes::<res_PInput>();
    
    IN_world.registerRes::<res_UIData>();
}