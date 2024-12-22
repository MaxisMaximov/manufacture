use super::*;

use components::*;
use types::{StyleSet, Vector2};
use vars::*;

use super::gmPrefab;

pub struct prefab_Player;
impl gmPrefab for prefab_Player{
    fn spawn(&self, IN_builder: gmObjBuilder) {
        IN_builder
            .addComp(comp_HP{val: 100})
            .addComp(comp_PController{active: true})
            .addComp(comp_Pos{x: 0, y: 0})
            .addComp(comp_Vel{x: 0, y: 0})
            .addComp(comp_Sprite{ sizeX: 1, sizeY: 1, sprite: vec![StyleSet{ ch: 'P', fg: Color::White, bg: Color::Cyan }], zDepth: 1 });
    }
}

pub struct prefab_GridWorldChunk{
   chunk: Vector2
}
impl gmPrefab for prefab_GridWorldChunk{
    fn spawn(&self, IN_builder: gmObjBuilder) {
        IN_builder
            .addComp(comp_Pos{ x: self.chunk.0 * CHUNK_X, y: self.chunk.1 * CHUNK_Y })
            .addComp(comp_TileTerrainChunk{ chunk: self.chunk })
            .addComp(comp_Sprite{ sizeX: CHUNK_X as usize, sizeY: CHUNK_Y as usize, sprite: vec![StyleSet{ ch: ' ', fg: Color::White, bg: Color::Black }], zDepth: 0 });
    }
}