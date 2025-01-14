use super::*;

use components::*;
use types::{Node, StyleSet, Vector2};
use vars::*;

use super::gmPrefab;

pub struct prefab_Player;
impl gmPrefab for prefab_Player{
    fn spawn(&self, IN_builder: gmObjBuilder) {
        IN_builder
            .addComp(comp_HP{val: 100})
            .addComp(comp_PController{active: true})
            .addComp(comp_Pos{x: 0, y: 0})
            .addComp(comp_Vel{x: 0, y: 0, frozen: false})
            .addComp(comp_Sprite{ sizeX: 1, sizeY: 1, sprite: vec![StyleSet{ ch: 'P', fg: Color::White, bg: Color::Cyan }], zDepth: 1 })
            .finish();
    }
}

pub struct prefab_GridWorldChunk{
   pub chunk: Vector2,
}
impl gmPrefab for prefab_GridWorldChunk{
    fn spawn(&self, IN_builder: gmObjBuilder) {
        IN_builder
            .addComp(comp_Pos{ x: ((self.chunk.0 * CHUNK_X) + CHUNK_X/2), y: ((self.chunk.1 * CHUNK_Y) + CHUNK_Y/2) - 1})
            .addComp(comp_TileTerrainChunk{ chunk: self.chunk, fresh: true })
            .addComp(comp_Sprite{ sizeX: CHUNK_X as usize, sizeY: CHUNK_Y as usize, sprite: vec![StyleSet{ ch: '0', fg: Color::Black, bg: Color::White }; CHUNK_X as usize * CHUNK_Y as usize], zDepth: 0 })
        .finish();
    }
}
impl prefab_GridWorldChunk{
    pub fn new(IN_chunk: Vector2) -> Self{
        Self{
            chunk: IN_chunk,
        }
    }
}

pub struct idkfa_UI{}
impl gmPrefab for idkfa_UI{
    fn spawn(&self, IN_builder: gmObjBuilder) {
        use resources::ui::*;

        IN_builder
            .addComp(comp_UIBox{
                position: (0, 0),
                elements: Node::new(
                    UI_element{
                        type_: UI_type::special(
                            Box::new(
                                specials::UISpec_progressBar{
                                    length: 8,
                                    maxVal: 20,
                                    trackVal: "idkfa".to_owned(),
                        })),
                        style: UI_style{
                            position: UI_pos::Rel((-5, 5)),
                            size: UI_size::Frac((50, 50)),
                            fg: Color::White,
                            bg: Color::Black,
                            border: UI_border::fancy,
                        },
                    }, 0, 5),
            })
            .finish();
    }
}