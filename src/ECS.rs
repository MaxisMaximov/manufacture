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
    fn spawn(&self, IN_id: &u16, IN_compMapRef: &mut WORLD_compMap);
}

pub struct gmObjPrefP{
    health: u16,
    position: types::vector2,
    velocity: types::vector2,
    render: gmCompRender,
    Pcontrol: bool
}
impl Default for gmObjPrefP{
    fn default() -> Self {
        Self{
            health: 100,
            position: (0, 0),
            velocity: (0, 0),
            render: gmCompRender{
                size: (1, 1),
                sprite: &[types::styleSet{
                    ch: 'P',
                    fg: Color::White,
                    bg: Color::Cyan
                    }
                ],
                visible: true,
            },
            Pcontrol: true
        }
    }
}
impl gmObjPrefEx for gmObjPrefP{
    fn spawn(self, IN_id: &u16, IN_compMapRef: &mut WORLD_compMap){

        IN_compMapRef.get(gmCompHealth::COMP_ID())
            .unwrap().insert(IN_id, gmCompHealth{val: self.health});

        IN_compMapRef.get(gmCompPosition::COMP_ID())
            .unwrap().insert(IN_id, gmCompPosition{x: self.position.0, y: self.position.1});

        IN_compMapRef.get(gmCompVelocity::COMP_ID())
            .unwrap().insert(IN_id, gmCompVelocity{x: self.velocity.0, y: self.velocity.1});

        IN_compMapRef.get(gmCompRender::COMP_ID())
            .unwrap().insert(IN_id, self.render);

        IN_compMapRef.get(gmCompPController::COMP_ID())
            .unwrap().insert(IN_id, gmCompPController{active: self.Pcontrol});

    }
}
