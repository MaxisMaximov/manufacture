use std::collections::BTreeMap;

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
    type COMP_STORAGE = sMBTreeMap<Self>;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompHealth"
    }
}

pub struct gmCompPosition{
    x: usize,
    y: usize
}
impl gmCompEx for gmCompPosition{
    type COMP_STORAGE = sMBTreeMap<Self>;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompPosition"
    }
}

pub struct gmCompVelocity{
    x: usize,
    y: usize
}
impl gmCompEx for gmCompVelocity{
    type COMP_STORAGE = sMDenseVec<Self>;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompVelocity"
    }
}

pub struct gmCompTerrainChunk{
    cells: [types::styleSet; vars::WORLD::GENERAL::CHUNK_X * vars::WORLD::GENERAL::CHUNK_Y],
    needsUpdate: bool
}
impl gmCompEx for gmCompTerrainChunk{
    type COMP_STORAGE = sMDenseVec<Self>;

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
    type COMP_STORAGE = sMBTreeMap<Self>;

    fn COMP_ID() -> &'static str {
        "manufacture::gmCompRender"
    }
}

pub struct gmCompPController{
    active: bool
}
impl gmCompEx for gmCompPController{
    type COMP_STORAGE = sMBTreeMap<Self>;

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
    // "`IN_prefab` is defined here, but is not a type"
    // WHAT??
    // RUSTC WHAT THE PECK DO YOU MEAN????
    pub fn fromPrefab<T>(mut self, IN_prefab: &T) -> Self where T: gmObjPrefEx{
        IN_prefab::spawn(self.compMapRef);
        self
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


pub trait gmStorageEx{
    type outputType;
    fn new() -> Self;
    fn get(&self, IN_id: &u16) -> Option<&Self::outputType>;
    fn getMut(&mut self, IN_id: &u16) -> Option<&mut Self::outputType>;
    fn insert(&mut self, IN_id: u16, IN_item: Self::outputType);
    fn remove(&mut self, IN_id: &u16);
    fn iter(&self) -> impl Iterator;
    fn iterMut(&mut self) -> impl Iterator;
}
pub trait gmStorageBox{}
impl<T> gmStorageBox for T where T: gmStorageEx{}

pub struct sMBTreeMap<T>{
    innerMap: BTreeMap<u16, T>,
}
impl<T> gmStorageEx for sMBTreeMap<T>{

    type outputType = T;

    fn new() -> Self{
        Self{
            innerMap: BTreeMap::new()
        }
    }

    fn get(&self, IN_id: &u16) -> Option<&Self::outputType> {
        self.innerMap.get(IN_id)
    }

    fn getMut(&mut self, IN_id: &u16) -> Option<&mut Self::outputType> {
        self.innerMap.get_mut(IN_id)
    }

    fn insert(&mut self, IN_id: u16, IN_item: Self::outputType) {
        self.innerMap.insert(IN_id, IN_item);
    }

    fn remove(&mut self, IN_id: &u16) {
        self.innerMap.remove(IN_id);
    }

    fn iter(&self) -> impl Iterator {
        self.innerMap.iter()
    }

    fn iterMut(&mut self) -> impl Iterator {
        self.innerMap.iter_mut()
    }
    
}

pub struct sMHashMap<T>{
    innerMap: HashMap<u16, T>
}
impl<T> gmStorageEx for sMHashMap<T>{
    type outputType = T;

    fn new() -> Self {
        Self{
            innerMap: HashMap::new()
        }
    }

    fn get(&self, IN_id: &u16) -> Option<&Self::outputType> {
        self.innerMap.get(IN_id)
    }

    fn getMut(&mut self, IN_id: &u16) -> Option<&mut Self::outputType> {
        self.innerMap.get_mut(IN_id)
    }

    fn insert(&mut self, IN_id: u16, IN_item: Self::outputType) {
        self.innerMap.insert(IN_id, IN_item);
    }

    fn remove(&mut self, IN_id: &u16){
        self.innerMap.remove(IN_id);
    }

    fn iter(&self) -> impl Iterator {
        self.innerMap.iter()
    }

    fn iterMut(&mut self) -> impl Iterator {
        self.innerMap.iter_mut()
    }
}

pub struct sMDenseVec<T>{
    innerProxyMap: HashMap<u16, usize>,
    innerDenseVec: Vec<sMDenseVecIndex<T>>
}

pub struct sMDenseVecIndex<T>{
    superID: u16,
    val: T
}
impl<T> gmStorageEx for sMDenseVec<T>{
    type outputType = T;

    fn new() -> Self {
        Self{
            innerProxyMap: HashMap::new(),
            innerDenseVec: Vec::new()
        }
    }

    fn get(&self, IN_id: &u16) -> Option<&Self::outputType> {
        if let Some(INDEX) = self.innerProxyMap.get(IN_id){
            return Some(&self.innerDenseVec[*INDEX].val);
        };
        None
    }

    fn getMut(&mut self, IN_id: &u16) -> Option<&mut Self::outputType> {
        if let Some(INDEX) = self.innerProxyMap.get(IN_id){
            return Some(&mut self.innerDenseVec[*INDEX].val);
        };
        None
    }

    fn insert(&mut self, IN_id: u16, IN_item: Self::outputType) {
        self.innerProxyMap.insert(IN_id, self.innerDenseVec.len()); // Length is always guaranteed to be next free index in Vec
        self.innerDenseVec.push(sMDenseVecIndex{superID: IN_id, val: IN_item});
    }

    fn remove(&mut self, IN_id: &u16){
        // Check if it's in the ProxyMap
        if let Some(INDEX) = self.innerProxyMap.remove(IN_id){
            // If it's the last index, just pop it
            if INDEX == self.innerDenseVec.len() - 1{
                self.innerDenseVec.pop();
                return
            }
            // If not, Clone the value to an idkfa value
            let idkfa = self.innerDenseVec.get(INDEX).unwrap().superID;
            // Update the index in ProxyMap
            *self.innerProxyMap.get_mut(&idkfa).unwrap() = *IN_id as usize;
                // Take the superID of the value
                // Find it in ProxyMap
                // Update the ProxyMap value with the removal index
            // Swap Removal and last index (Pop)
            self.innerDenseVec[INDEX] = self.innerDenseVec.pop().unwrap();
        }
    }

    fn iter(&self) -> impl Iterator {
        self.innerDenseVec.iter()
    }

    fn iterMut(&mut self) -> impl Iterator {
        self.innerDenseVec.iter_mut()
    }
}


pub trait gmEventEx{
    fn EVENT_ID() -> &str;
}
pub trait gmEventBox{}
impl<T> gmEventBox for T where T: gmEventEx{}