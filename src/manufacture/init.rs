use super::*;

use prefabs::*;
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
    IN_world.registerRes::<res_GridWorld>();
    IN_world.registerRes::<res_PID>();
    IN_world.registerRes::<res_PInput>();
    IN_world.registerRes::<res_UIData>();
    IN_world.registerRes::<res_LoadedChunks>();

    // Register Events
    IN_world.registerEvent::<event_TileChange>();
    IN_world.registerEvent::<event_BatchTileChange>();

    prefab_Player::spawn(&prefab_Player{}, IN_world.createGmObj());

    prefab_GridWorldChunk::spawn(&prefab_GridWorldChunk{ chunk: (0, 0), color: Color::Blue}, IN_world.createGmObj());
    prefab_GridWorldChunk::spawn(&prefab_GridWorldChunk{ chunk: (1, 0), color: Color::Cyan}, IN_world.createGmObj());
    prefab_GridWorldChunk::spawn(&prefab_GridWorldChunk{ chunk: (0, 1), color: Color::DarkBlue}, IN_world.createGmObj());
    prefab_GridWorldChunk::spawn(&prefab_GridWorldChunk{ chunk: (-1, 0), color: Color::DarkCyan}, IN_world.createGmObj());
    prefab_GridWorldChunk::spawn(&prefab_GridWorldChunk{ chunk: (0, -1), color: Color::DarkGreen}, IN_world.createGmObj());
    prefab_GridWorldChunk::spawn(&prefab_GridWorldChunk{ chunk: (1, 1), color: Color::DarkGrey}, IN_world.createGmObj());
    prefab_GridWorldChunk::spawn(&prefab_GridWorldChunk{ chunk: (1, -1), color: Color::DarkMagenta}, IN_world.createGmObj());
    prefab_GridWorldChunk::spawn(&prefab_GridWorldChunk{ chunk: (-1, -1), color: Color::DarkRed}, IN_world.createGmObj());
    prefab_GridWorldChunk::spawn(&prefab_GridWorldChunk{ chunk: (-1, 1), color: Color::DarkYellow}, IN_world.createGmObj());

    IN_world.fetchResMut::<res_PID>().insert(1, 0);

    IN_world.createGmObj().addComp(comp_ViewportCamera{ trackedEntity: 0, offset: (0, 0), active: true });
}