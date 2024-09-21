use super::*;

pub trait gmCompEx{
    type COMP_STORAGE: gmStorageEx;
    fn COMP_ID() -> &'static str;
}
pub trait gmCompBox{}
impl<T> gmCompBox for T where T: gmCompEx{}

pub struct gmCompHealth{
    val: u16
}
impl gmCompEx for gmCompHealth{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompHealth"
    }
}

pub struct gmCompPosition{
    x: usize,
    y: usize
}
impl gmCompEx for gmCompPosition{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompPosition"
    }
}

pub struct gmCompVelocity{
    x: usize,
    y: usize
}
impl gmCompEx for gmCompVelocity{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompVelocity"
    }
}

pub struct gmCompTerrainChunk{
    cells: [types::styleSet; vars::WORLD::GENERAL::CHUNK_X * vars::WORLD::GENERAL::CHUNK_Y],
    needsUpdate: bool
}
impl gmCompEx for gmCompTerrainChunk{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompTerrainChunk"
    }
}

pub struct gmCompRender{
    size: types::vector2,
    sprite: &'static [types::styleSet],
    visible: bool
}
impl gmCompEx for gmCompRender{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompRender"
    }
}

pub struct gmCompPController{
    active: bool
}
impl gmCompEx for gmCompPController{
    type COMP_STORAGE;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompPController"   
    }
}


pub struct gmObj{}

pub struct gmObjBuilder<'a>{
    ID: u16,
    compMapRef: &'a mut WORLD_compMap
}
impl gmObjBuilder<'_>{
    pub fn new(IN_id: u16, IN_compMapRef: &mut WORLD_compMap) -> Self{
        Self{
            ID: IN_id,
            compMapRef: IN_compMapRef,
        }
    }
    pub fn addComp<T>(self, IN_comp: T) -> Self where T: gmCompEx{
        self.compMapRef.get(&T::COMP_ID()).unwrap().insert(&self.ID, T);
        self
    }
    pub fn finish(self) -> u16{
        self.ID
    }
    pub fn fromPrefab<T>(mut self, IN_prefab: &T) -> Self where T: gmObjPrefEx{
        IN_prefab::spawn(self.compMapRef)
    }
}

pub trait gmObjPrefEx: Default{
    fn spawn(&self, IN_compMapRef: &mut WORLD_compMap);
}