use super::*;

use components::*;
use types::StyleSet;

use super::gmPrefab;

pub struct prefab_Player{

}
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