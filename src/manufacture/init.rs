use super::*;

use prefabs::*;
use systems::*;
use resources::*;
use components::*;

pub fn init(IN_world: &mut gmWorld, IN_dispatch: &mut gmDispatcher){
    // Register Systems
    IN_dispatch.addSys::<SysInput>();
    IN_dispatch.addSys::<SysPmove>();
    IN_dispatch.addSys::<SysMove>();
    IN_dispatch.addSys::<SysPtileChange>();
    IN_dispatch.addSys::<SysTileChunkUpdate>();
    IN_dispatch.addSys::<SysTileChunkSpriteUpdate>();
    IN_dispatch.addSys::<SysPchunkUnLoad>();
    IN_dispatch.addSys::<SysRenderer>();

    // Register Components
    IN_world.registerComp::<CompHp>();
    IN_world.registerComp::<CompPcontroller>();
    IN_world.registerComp::<CompPos>();
    IN_world.registerComp::<CompVel>();
    IN_world.registerComp::<CompSprite>();
    IN_world.registerComp::<CompTileTerrainChunk>();
    IN_world.registerComp::<CompGUI>();
    IN_world.registerComp::<CompViewportCamera>();

    // Register Resources
    IN_world.registerRes::<res_DeltaT>();
    IN_world.registerRes::<res_GridWorld>();
    IN_world.registerRes::<res_PID>();
    IN_world.registerRes::<res_PInput>();
    IN_world.registerRes::<res_UIData>();
    IN_world.registerRes::<res_LoadedChunks>();

    // Register Events
    IN_world.registerEvent::<event_TileChange>();
    IN_world.registerEvent::<event_BatchTileChange>();

    PrefabPlayer::spawn(&PrefabPlayer{}, IN_world.createGmObj());

    IN_world.fetchResMut::<res_PID>().insert(1, 0);

    IN_world.createGmObj().addComp(CompViewportCamera{ tracked_entity: 0, offset: (0, 0), active: true }).finish();
    
    IdkfaUi::spawn(&IdkfaUi{}, IN_world.createGmObj());
}