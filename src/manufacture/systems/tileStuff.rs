use prefabs::prefab_GridWorldChunk;

use super::*;


pub struct sys_PTileChange{}
impl<'a> gmSystem<'a> for sys_PTileChange{
    type sysData = sysData_PTileChange<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_PTileChange"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        if let Some(PID) = IN_data.res_PID.get(&1){

            let w_PCoords = IN_data.comp_Pos.get(PID).expect(&format!("ERROR: Player {PID} has no Position component"));
            let w_oldTile = IN_data.res_World.getTile((w_PCoords.x, w_PCoords.y)).expect(&format!("ERROR: Tile at {}, {} doesn't exist", w_PCoords.x, w_PCoords.y)).mat;

            match IN_data.res_PInput.code{
                KeyCode::Char('f') => {
                    IN_data.event_TileChange.push(event_TileChange{
                        coords: (w_PCoords.x, w_PCoords.y),
                        newTile: w_oldTile + 1,
                    });
                }
                KeyCode::Char('F') => {
                    IN_data.event_TileChange.push(event_TileChange{
                        coords: (w_PCoords.x, w_PCoords.y),
                        newTile: w_oldTile - 1,
                    });
                }
                KeyCode::Char('g') => {
                    IN_data.event_TileChange.push(event_TileChange{
                        coords: (w_PCoords.x, w_PCoords.y),
                        newTile: 0,
                    });
                }
                _ => {return}
            }
        }
    }
}
pub struct sysData_PTileChange<'a>{
    res_World: Fetch<'a, res_GridWorld>,
    res_PInput: Fetch<'a, res_PInput>,
    res_PID: Fetch<'a, res_PID>,
    event_TileChange: EventWriter<'a, event_TileChange>,
    comp_Pos: ReadStorage<'a, comp_Pos>,
}
impl<'a> gmSystemData<'a> for sysData_PTileChange<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            res_World: IN_world.fetchRes(),
            res_PInput: IN_world.fetchRes(),
            res_PID: IN_world.fetchRes(),
            event_TileChange: IN_world.fetchEventWriter(),
            comp_Pos: IN_world.fetch()
        }
    }
}

pub struct sys_TileChunkUpdate{}
impl<'a> gmSystem<'a> for sys_TileChunkUpdate{
    type sysData = sysData_TileChunkUpdate<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_TileChunkUpdate"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {

        for EVENT in IN_data.event_TileChange.iter(){
            if let Some(TILE) = IN_data.res_World.getTileMut(EVENT.coords){
                TILE.mat = EVENT.newTile

            }
            if let Some(CHUNK) = IN_data.res_World.getChunkFromTileMut(EVENT.coords){
                CHUNK.needsResprite = true
            }
        }

        for EVENT in IN_data.event_BatchTileChange.iter(){

            for COORDS in IN_data.res_World.getChunkRange(EVENT.from, EVENT.to).iter(){

                if let Some(w_chunk) = IN_data.res_World.getChunkMut(*COORDS){
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
pub struct sysData_TileChunkUpdate<'a>{
    pub event_TileChange: EventReader<'a, event_TileChange>,
    pub event_BatchTileChange: EventReader<'a, event_BatchTileChange>,
    pub res_World: FetchMut<'a, res_GridWorld>
}
impl<'a> gmSystemData<'a> for sysData_TileChunkUpdate<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            event_TileChange: IN_world.fetchEventReader(),
            event_BatchTileChange: IN_world.fetchEventReader(),
            res_World: IN_world.fetchResMut(),
        }
    }
}

pub struct sys_TileChunkSpriteUpdate{}
impl<'a> gmSystem<'a> for sys_TileChunkSpriteUpdate{
    type sysData = sysData_TileChunkSpriteUpdate<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_TileChunkSpriteUpdate"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        for denseVecEntry{id: GMOBJID, val: CHUNKCOMP} in IN_data.comp_TileTerrain.inner.iter(){
            if let Some(CHUNK) = IN_data.res_World.getChunkMut(CHUNKCOMP.chunk){
                if !CHUNK.needsResprite && !CHUNKCOMP.fresh {continue} // Skip if it doesn't need a resprite or it's not a fresh chunk

                let idkfa = IN_data.comp_Sprite.get_mut(GMOBJID).expect(&format!("ERROR: Chunk at {}, {} has no Sprite component", CHUNKCOMP.chunk.0, CHUNKCOMP.chunk.1));
                let w_chunkSprite = idkfa.sprite.chunks_mut(idkfa.sizeX).rev();
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
pub struct sysData_TileChunkSpriteUpdate<'a>{
    pub comp_TileTerrain: ReadStorage<'a, comp_TileTerrainChunk>,
    pub comp_Sprite: WriteStorage<'a, comp_Sprite>,
    pub res_World: FetchMut<'a, res_GridWorld>
}
impl<'a> gmSystemData<'a> for sysData_TileChunkSpriteUpdate<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_TileTerrain: IN_world.fetch(),
            comp_Sprite: IN_world.fetchMut(),
            res_World: IN_world.fetchResMut(),
        }
    }
}

pub struct sys_PChunkUnLoad{
    oldChunk: Vector2
}
impl<'a> gmSystem<'a> for sys_PChunkUnLoad{
    type sysData = sysData_PChunkUnLoad<'a>;

    const sysDepends: &'static [&'static str] = &[];

    fn new() -> Self {
        Self{
            oldChunk: (0, 0),
        }
    }

    fn SYS_ID() -> &'static str {
        "sys_PChunkUnLoad"
    }
    
    fn execute(&mut self, mut IN_data: Self::sysData) {
        // First update the chunk list
        for CHUNK in IN_data.comp_TileTerrainChunk.inner.iter(){
            IN_data.res_LoadedChunks.entry(CHUNK.val.chunk).or_insert(CHUNK.id);
        }

        // Get player position
        let PID = IN_data.res_PID.get(&1).expect("ERROR: Player object not present");
        let idkfa_PPos = IN_data.comp_Pos.get(PID).expect(&format!("ERROR: Player {PID} has no Position component"));
        let w_PChunk = utils::util_coordConvert((idkfa_PPos.x, idkfa_PPos.y)).0;
        
        // Don't bother if player hasn't moved from their chunk
        if w_PChunk == self.oldChunk{
            return
        }
        self.oldChunk = w_PChunk;

        // Set Discarded Vec
        let mut w_discardedChunks: Vec<(Vector2, gmID)> = Vec::new();

        for (CHUNK, GMID) in IN_data.res_LoadedChunks.iter_mut(){

            // If the chunk is too far away from player chunk, push it to discarded
            if CHUNK.0.abs_diff(w_PChunk.0) > CHUNK_UNLOAD_MARGIN || CHUNK.1.abs_diff(w_PChunk.1) > CHUNK_UNLOAD_MARGIN{
                w_discardedChunks.push((*CHUNK, *GMID));
            }
        }

        // Send a command to remove the discarded chunks and remove them from the HashMap
        for (CHUNK, GMID) in w_discardedChunks.iter(){
            IN_data.cmdqueue.push(Box::new(cmd_DespawnGmObj{id: *GMID}));
            IN_data.res_LoadedChunks.remove(CHUNK);
        }

        // And send commands to spawn new visible chunks
        for XPOS in (w_PChunk.0 - CHUNK_UNLOAD_MARGIN as isize)..(w_PChunk.0 + CHUNK_UNLOAD_MARGIN as isize){
            for YPOS in (w_PChunk.1 - CHUNK_UNLOAD_MARGIN as isize)..(w_PChunk.1 + CHUNK_UNLOAD_MARGIN as isize){
                if !IN_data.res_LoadedChunks.contains_key(&(XPOS, YPOS)){
                    // Unfortunately I can't update the LoadedChunks hashmap right away as I don't know what IDs the new chunks will get
                    IN_data.cmdqueue.push(Box::new(cmd_spawnPrefab{prefab: prefab_GridWorldChunk::new((XPOS, YPOS))}));
                }
            }
        }
        
    }
}
pub struct sysData_PChunkUnLoad<'a>{
    pub comp_Pos: ReadStorage<'a, comp_Pos>,
    pub comp_TileTerrainChunk: ReadStorage<'a, comp_TileTerrainChunk>,
    pub res_PID: Fetch<'a, res_PID>,
    pub res_LoadedChunks: FetchMut<'a, res_LoadedChunks>,
    pub cmdqueue: FetchMut<'a, gmWorld_CMDQUEUE>
}
impl<'a> gmSystemData<'a> for sysData_PChunkUnLoad<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self {
        Self{
            comp_Pos: IN_world.fetch(),
            comp_TileTerrainChunk: IN_world.fetch(),
            res_PID: IN_world.fetchRes(),
            res_LoadedChunks: IN_world.fetchResMut(),
            cmdqueue: IN_world.fetchCommandWriter(),
        }
    }
}