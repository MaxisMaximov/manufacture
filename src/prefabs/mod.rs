use super::*;

use components::*;
use types::{Node, StyleSet, Vector2};
use constants::*;

use super::gmPrefab;

pub struct PrefabPlayer;
impl gmPrefab for PrefabPlayer{
    fn spawn(&self, IN_builder: gmObjBuilder) {
        IN_builder
            .addComp(CompHp{val: 100})
            .addComp(CompPcontroller{active: true})
            .addComp(CompPos{x: 0, y: 0})
            .addComp(CompVel{x: 0, y: 0, frozen: false})
            .addComp(CompSprite{ size_x: 1, size_y: 1, sprite: vec![StyleSet{ ch: 'P', fg: Color::White, bg: Color::Cyan }], z_depth: 1 })
            .finish();
    }
}

pub struct PrefabGridWorldChunk{
   pub chunk: Vector2,
}
impl gmPrefab for PrefabGridWorldChunk{
    fn spawn(&self, IN_builder: gmObjBuilder) {
        IN_builder
            .addComp(CompPos{ x: ((self.chunk.0 * CHUNK_X) + CHUNK_X/2), y: ((self.chunk.1 * CHUNK_Y) + CHUNK_Y/2) - 1})
            .addComp(CompTileTerrainChunk{ chunk: self.chunk, fresh: true })
            .addComp(CompSprite{ size_x: CHUNK_X as usize, size_y: CHUNK_Y as usize, sprite: vec![StyleSet{ ch: '0', fg: Color::Black, bg: Color::White }; CHUNK_X as usize * CHUNK_Y as usize], z_depth: 0 })
        .finish();
    }
}
impl PrefabGridWorldChunk{
    pub fn new(IN_chunk: Vector2) -> Self{
        Self{
            chunk: IN_chunk,
        }
    }
}

pub struct IdkfaUi{}
impl gmPrefab for IdkfaUi{
    fn spawn(&self, IN_builder: gmObjBuilder) {
        use resources::ui::*;

        IN_builder.addComp(CompGUI{
            position: (0, 0),
            elements: Node::new(
                UIElement{
                    tag: UITag::Special(
                        Box::new(
                            specials::UISpecProgressBar{
                                length: 8,
                                max_val: 20,
                                track_val: "idkfa".to_owned(),
                    })),
                    style: UIStyle{
                        position: UIPos::Rel((-5, 5)),
                        size: UISize::Frac((50, 50)),
                        fg: Color::White,
                        bg: Color::Black,
                        border: UIBorder::Fancy,
                        display: UIDisplay::Float
                    },
                }, 0),
        }).finish();
    }
}