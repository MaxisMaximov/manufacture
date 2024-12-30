use super::*;

pub struct event_TileChange{
    pub coords: Vector2,
    pub newTile: u8
}
impl gmEvent for event_TileChange{
    fn EVENT_ID() -> &'static str {
        "event_TileChange"
    }
}

pub struct event_BatchTileChange{
    pub from: Vector2,
    pub to: Vector2,
    pub newTile: u8
}
impl gmEvent for event_BatchTileChange{
    fn EVENT_ID() -> &'static str {
        "event_BatchTileChange"
    }
}

pub struct event_InvOp_AddItem{
    pub item: types::inv_Item,
    pub inventory: gmID
}