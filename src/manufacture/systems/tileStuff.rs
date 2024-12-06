use super::*;


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
        let w_PCoords = IN_data.comp_Pos.get(IN_data.res_PID.res.get(&1).unwrap());
        let w_oldTile = IN_data.res_World.getTile((w_PCoords.x, w_PCoords.y)).unwrap().mat;

        match IN_data.res_PInput.res.code{
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
pub struct sysData_PTileChange<'a>{
    res_World: Fetch<'a, res_GridWorld>,
    res_PInput: Fetch<'a, res_PInput>,
    res_PID: Fetch<'a, res_PID>,
    event_TileChange: FetchMut<'a, Vec<event_TileChange>>,
    comp_Pos: readStorage<'a, comp_Pos>,
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
    pub event_TileChange: Fetch<'a, Vec<event_TileChange>>,
    pub event_BatchTileChange: Fetch<'a, Vec<event_BatchTileChange>>,
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

    fn new() -> Self {
        Self{}
    }

    fn SYS_ID() -> &'static str {
        "sys_TileChunkSpriteUpdate"
    }

    fn execute(&mut self, mut IN_data: Self::sysData) {
        for denseVecEntry{id: GMOBJID, val: CHUNKCOMP} in IN_data.comp_TileTerrain.inner.iter(){
            if let Some(CHUNK) = IN_data.res_World.getChunkMut(CHUNKCOMP.chunk){
                if !CHUNK.needsResprite{continue}

                let w_chunkSprite = IN_data.comp_Sprite.get_mut(GMOBJID);

                for (INDEX, PIXEL) in w_chunkSprite.sprite.iter_mut().enumerate(){
                    *PIXEL = match CHUNK.cells[INDEX].mat{
                        0 => {StyleSet{ ch: ' ', fg: Color::Black, bg: Color::Black }} // Empty
                        1 => {StyleSet{ ch: 'w', fg: Color::White, bg: Color::Blue }} // Water
                        2 => {StyleSet{ ch: 't', fg: Color::White, bg: Color::Green }} // Tree
                        _ => {StyleSet{ ch: '0', fg: Color::Black, bg: Color::White }} // UNKNOWN
                    }
                }

                CHUNK.needsResprite = false;
            }
        }
    }
}
pub struct sysData_TileChunkSpriteUpdate<'a>{
    pub comp_TileTerrain: readStorage<'a, comp_TileTerrainChunk>,
    pub comp_Sprite: writeStorage<'a, comp_Sprite>,
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