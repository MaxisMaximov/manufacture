// # Prefabs module
// This is where you define prefabs for easier instantiation
use super::*;

pub struct Player{
    pub loc: Vector2
}
impl EntityPrefab for Player{
    fn spawn(&self, builder: manufacture_engine::ECS::entity::EntityBuilder<'_>) {
        builder.with(Transform2D{
            loc: self.loc,
            rot: 0.0,
            scale: Vector2 { x: 1.0, y: 1.0 },
        })
        .with(CMDSprite{
            id: "Player1".to_owned(),
            z_index: 1.0,
        })
        .with(PlayerController {
            pid: 1,
            active: true
        }).finish();
    }
}

pub struct Crafter{
    pub loc: Vector2
}
impl EntityPrefab for Crafter{
    fn spawn(&self, builder: manufacture_engine::ECS::entity::EntityBuilder<'_>) {
        builder.with(Transform2D{
            loc: self.loc,
            rot: 0.0,
            scale: Vector2 { x: 1.0, y: 1.0 },
        })
        .with(CMDSprite{
            id: "Crafter".to_owned(),
            z_index: 1.0,
        }).finish();
    }
}

pub struct Storage{
    pub loc: Vector2
}
impl EntityPrefab for Storage{
    fn spawn(&self, builder: manufacture_engine::ECS::entity::EntityBuilder<'_>) {
        builder.with(Transform2D{
            loc: self.loc,
            rot: 0.0,
            scale: Vector2 { x: 1.0, y: 1.0 },
        })
        .with(CMDSprite{
            id: "Storage".to_owned(),
            z_index: 1.0,
        }).finish();
    }
}