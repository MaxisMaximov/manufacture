use prefabs::PrefabGridWorldChunk;

use super::*;


pub struct SysPtileChange{}
impl<'a> gmSystem<'a> for SysPtileChange{
    type sysData = SysDataPtileChange<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_PTileChange"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        if let Some(PID) = IN_data.res_pid.get(&1){

            let w_PCoords = IN_data.comp_pos.get(PID).expect(&format!("ERROR: Player {PID} has no Position component"));
            let w_oldTile = IN_data.res_world.getTile((w_PCoords.x, w_PCoords.y)).expect(&format!("ERROR: Tile at {}, {} doesn't exist", w_PCoords.x, w_PCoords.y)).mat;

            match IN_data.res_pinput.code{
                KeyCode::Char('f') => {
                    IN_data.event_tile_change.push(event_TileChange{
                        coords: (w_PCoords.x, w_PCoords.y),
                        newTile: w_oldTile + 1,
                    });
                }
                KeyCode::Char('F') => {
                    IN_data.event_tile_change.push(event_TileChange{
                        coords: (w_PCoords.x, w_PCoords.y),
                        newTile: w_oldTile - 1,
                    });
                }
                KeyCode::Char('g') => {
                    IN_data.event_tile_change.push(event_TileChange{
                        coords: (w_PCoords.x, w_PCoords.y),
                        newTile: 0,
                    });
                }
                _ => {return}
            }
        }
    }
}
pub struct SysDataPtileChange<'a>{
    res_world: Fetch<'a, res_GridWorld>,
    res_pinput: Fetch<'a, res_PInput>,
    res_pid: Fetch<'a, res_PID>,
    event_tile_change: EventWriter<'a, event_TileChange>,
    comp_pos: ReadStorage<'a, CompPos>,
}
impl<'a> gmSystemData<'a> for SysDataPtileChange<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_world: IN_world.fetchRes(),
            res_pinput: IN_world.fetchRes(),
            res_pid: IN_world.fetchRes(),
            event_tile_change: IN_world.fetchEventWriter(),
            comp_pos: IN_world.fetch()
        }
    }
}

pub struct SysTileChunkUpdate{}
impl<'a> gmSystem<'a> for SysTileChunkUpdate{
    type sysData = SysDataTileChunkUpdate<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_TileChunkUpdate"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {

        for EVENT in IN_data.event_tile_change.iter(){
            if let Some(TILE) = IN_data.res_world.getTileMut(EVENT.coords){
                TILE.mat = EVENT.newTile

            }
            if let Some(CHUNK) = IN_data.res_world.getChunkFromTileMut(EVENT.coords){
                CHUNK.needsResprite = true
            }
        }

        for EVENT in IN_data.event_batch_tile_change.iter(){

            for COORDS in IN_data.res_world.getChunkRange(EVENT.from, EVENT.to).iter(){

                if let Some(w_chunk) = IN_data.res_world.getChunkMut(*COORDS){
                    use std::cmp::{max, min};
                    // This is a mess but I calculated EVERYTHING for 2 hours straight for it to work
                    // Basically constraints iterator to chunk's boundaries and local coordinates

                    // Check latest comment in #22 for calculations

                    // Listen chunk iterators are better than accessing constantly from world level
                    for XPOS in
                        (max(EVENT.from.0, COORDS.0*6)) - COORDS.0*6 // FROM: Whichever is closer to 0
                        ..(min(EVENT.to.0+1, COORDS.0*6+6)) - COORDS.0*6 // TO: Whichever is closer to 0
                        {
                            for YPOS in // Same stuff
                                (max(EVENT.from.1, COORDS.1*6)) - COORDS.1*6
                                ..(min(EVENT.to.1+1, COORDS.1*6+6)) - COORDS.1*6{
                                    w_chunk[(XPOS, YPOS)].mat = EVENT.newTile
                                }
                    };
                    
                    w_chunk.needsResprite = true;
                }
            }
        }
    }
}
pub struct SysDataTileChunkUpdate<'a>{
    pub event_tile_change: EventReader<'a, event_TileChange>,
    pub event_batch_tile_change: EventReader<'a, event_BatchTileChange>,
    pub res_world: FetchMut<'a, res_GridWorld>
}
impl<'a> gmSystemData<'a> for SysDataTileChunkUpdate<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            event_tile_change: IN_world.fetchEventReader(),
            event_batch_tile_change: IN_world.fetchEventReader(),
            res_world: IN_world.fetchResMut(),
        }
    }
}

pub struct SysTileChunkSpriteUpdate{}
impl<'a> gmSystem<'a> for SysTileChunkSpriteUpdate{
    type sysData = SysDataTileChunkSpriteUpdate<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_TileChunkSpriteUpdate"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        for denseVecEntry{id: GMOBJID, val: CHUNKCOMP} in IN_data.comp_tile_terrain.inner.iter(){
            if let Some(CHUNK) = IN_data.res_world.getChunkMut(CHUNKCOMP.chunk){
                if !CHUNK.needsResprite && !CHUNKCOMP.fresh {continue} // Skip if it doesn't need a resprite or it's not a fresh chunk

                let idkfa = IN_data.comp_sprite.get_mut(GMOBJID).expect(&format!("ERROR: Chunk at {}, {} has no Sprite component", CHUNKCOMP.chunk.0, CHUNKCOMP.chunk.1));
                let w_chunkSprite = idkfa.sprite.chunks_mut(idkfa.size_x).rev();
                let w_chunkTiles = CHUNK.cells.chunks(CHUNK_X as usize);

                for (PIXROW, CHROW) in w_chunkSprite.zip(w_chunkTiles){
                    for (PIXEL, CELL) in PIXROW.iter_mut().zip(CHROW){
                        *PIXEL = match CELL.mat{
                            0 => {StyleSet{ ch: ' ', fg: Color::Black, bg: Color::Black }} // Empty
                            1 => {StyleSet{ ch: 'w', fg: Color::White, bg: Color::Blue }} // Water
                            2 => {StyleSet{ ch: 't', fg: Color::White, bg: Color::Green }} // Tree
                            _ => {StyleSet{ ch: '0', fg: Color::Black, bg: Color::White }} // UNKNOWN
                        }
                    }
                }

                CHUNK.needsResprite = false;
            }
        }
    }
}
pub struct SysDataTileChunkSpriteUpdate<'a>{
    pub comp_tile_terrain: ReadStorage<'a, CompTileTerrainChunk>,
    pub comp_sprite: WriteStorage<'a, CompSprite>,
    pub res_world: FetchMut<'a, res_GridWorld>
}
impl<'a> gmSystemData<'a> for SysDataTileChunkSpriteUpdate<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_tile_terrain: IN_world.fetch(),
            comp_sprite: IN_world.fetchMut(),
            res_world: IN_world.fetchResMut(),
        }
    }
}

pub struct SysPchunkUnLoad{
    old_chunk: Vector2
}
impl<'a> gmSystem<'a> for SysPchunkUnLoad{
    type sysData = SysDataPchunkUnLoad<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{
            old_chunk: (0, 0),
        }
    }

    fn SYS_ID() -> &'static str {
        "sys_PChunkUnLoad"
    }
    
    fn execute(&mut self, mut IN_data: Self::sysData) {
        // First update the chunk list
        for CHUNK in IN_data.comp_tile_terrain_chunk.inner.iter(){
            IN_data.res_loaded_chunks.entry(CHUNK.val.chunk).or_insert(CHUNK.id);
        }

        // Get player position
        let PID = IN_data.res_pid.get(&1).expect("ERROR: Player object not present");
        let idkfa_PPos = IN_data.comp_pos.get(PID).expect(&format!("ERROR: Player {PID} has no Position component"));
        let w_PChunk = utils::util_coordConvert((idkfa_PPos.x, idkfa_PPos.y)).0;
        
        // Don't bother if player hasn't moved from their chunk
        if w_PChunk == self.old_chunk{
            return
        }
        self.old_chunk = w_PChunk;

        // Set Discarded Vec
        let mut w_discardedChunks: Vec<(Vector2, usize)> = Vec::new();

        for (CHUNK, GMID) in IN_data.res_loaded_chunks.iter_mut(){

            // If the chunk is too far away from player chunk, push it to discarded
            if CHUNK.0.abs_diff(w_PChunk.0) > CHUNK_UNLOAD_MARGIN || CHUNK.1.abs_diff(w_PChunk.1) > CHUNK_UNLOAD_MARGIN{
                w_discardedChunks.push((*CHUNK, *GMID));
            }
        }

        // Send a command to remove the discarded chunks and remove them from the HashMap
        for (CHUNK, GMID) in w_discardedChunks.iter(){
            IN_data.cmdqueue.push(Box::new(cmd_DespawnGmObj{id: *GMID}));
            IN_data.res_loaded_chunks.remove(CHUNK);
        }

        // And send commands to spawn new visible chunks
        for XPOS in (w_PChunk.0 - CHUNK_UNLOAD_MARGIN as isize)..(w_PChunk.0 + CHUNK_UNLOAD_MARGIN as isize){
            for YPOS in (w_PChunk.1 - CHUNK_UNLOAD_MARGIN as isize)..(w_PChunk.1 + CHUNK_UNLOAD_MARGIN as isize){
                if !IN_data.res_loaded_chunks.contains_key(&(XPOS, YPOS)){
                    // Unfortunately I can't update the LoadedChunks hashmap right away as I don't know what IDs the new chunks will get
                    IN_data.cmdqueue.push(Box::new(cmd_spawnPrefab{prefab: PrefabGridWorldChunk::new((XPOS, YPOS))}));
                }
            }
        }
        
    }
}
pub struct SysDataPchunkUnLoad<'a>{
    pub comp_pos: ReadStorage<'a, CompPos>,
    pub comp_tile_terrain_chunk: ReadStorage<'a, CompTileTerrainChunk>,
    pub res_pid: Fetch<'a, res_PID>,
    pub res_loaded_chunks: FetchMut<'a, res_LoadedChunks>,
    pub cmdqueue: FetchMut<'a, gmWorld_CMDQUEUE>
}
impl<'a> gmSystemData<'a> for SysDataPchunkUnLoad<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_pos: IN_world.fetch(),
            comp_tile_terrain_chunk: IN_world.fetch(),
            res_pid: IN_world.fetchRes(),
            res_loaded_chunks: IN_world.fetchResMut(),
            cmdqueue: IN_world.fetchCommandWriter(),
        }
    }
}