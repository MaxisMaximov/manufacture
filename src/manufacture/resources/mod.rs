use std::ops::{Index, IndexMut};

use super::*;

use vars::*;

pub struct res_PInput{
    pub res: KeyEvent
}
impl gmRes for res_PInput{
    fn new() -> Self {
        Self{
            res: KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
        }
    }

    fn RES_ID() -> &'static str {
        "res_PInput"
    }
}

pub struct res_DeltaT{
    pub res: Duration
}
impl gmRes for res_DeltaT{
    fn new() -> Self {
        Self{
            res: Duration::from_secs(0)
        }
    }

    fn RES_ID() -> &'static str {
        "res_DeltaT"
    }
}

pub struct res_Events{
    pub activeBuffer: bool,
    pub inner: HashMap<&'static str, (Box<dyn Any>, Box<dyn Any>)>,
}
impl res_Events{

    pub fn registerEvent<T>(&mut self) where T: gmEvent + 'static{
        self.inner.entry(T::EVENT_ID()).or_insert((Box::new(Vec::<T>::new()), Box::new(Vec::<T>::new())));
    }

    pub fn unRegisterEvent<T>(&mut self) where T: gmEvent + 'static{
        self.inner.remove(T::EVENT_ID());
    }

    fn getActiveBuffer<T>(&mut self) -> &mut Vec<T> where T: gmEvent + 'static{
        let BUFFERS = self.inner.get_mut(T::EVENT_ID()).unwrap();

        if self.activeBuffer{BUFFERS.1.downcast_mut::<Vec<T>>().unwrap()}
        else{BUFFERS.0.downcast_mut::<Vec<T>>().unwrap()}
    }

    fn getAlternateBuffer<T>(&mut self) -> &mut Vec<T> where T: gmEvent + 'static{
        let BUFFERS = self.inner.get_mut(T::EVENT_ID()).unwrap();

        if self.activeBuffer {BUFFERS.1.downcast_mut::<Vec<T>>().unwrap()}
        else{BUFFERS.0.downcast_mut::<Vec<T>>().unwrap()}
    }

    pub fn read<T>(&mut self) -> &Vec<T> where T: gmEvent + 'static{
        self.getActiveBuffer::<T>()
    }

    pub fn push<T>(&mut self, IN_event: T) where T: gmEvent + 'static{
        self.getAlternateBuffer().push(IN_event)
    }

    pub fn switchBuffer(&mut self){
        self.activeBuffer = !self.activeBuffer
    }
}
impl gmRes for res_Events{
    fn new() -> Self {
        Self{
            activeBuffer: false,
            inner: HashMap::new()
        }
    }

    fn RES_ID() -> &'static str {
        "res_Event"
    }
}
pub trait gmEvent{
    fn EVENT_ID() -> &'static str;
}

pub struct res_PID{
    pub res: HashMap<gmID, gmID> // PID, gmObjID
}
impl gmRes for res_PID{
    fn new() -> Self {
        Self{
            res: HashMap::new()
        }
    }

    fn RES_ID() -> &'static str {
        "res_PID"
    }
}

pub struct res_GridWorld{
    chunks: HashMap<Vector2, GridWorldChunk>
}
impl gmRes for res_GridWorld{
    fn new() -> Self {
        let mut idkfa_map: HashMap<(isize, isize), GridWorldChunk> = HashMap::new();

        for CH_X in WORLD_X_MIN..WORLD_X_MAX {
            for CH_Y in WORLD_Y_MIN..WORLD_Y_MAX{
                idkfa_map.insert((CH_X, CH_Y), GridWorldChunk::new());
            }
        }

        Self{
            chunks: idkfa_map
        }
    }

    fn RES_ID() -> &'static str {
        "res_GridWorld"
    }
}
impl res_GridWorld{
    pub fn getChunk(&self, IN_coords: Vector2) -> Option<&GridWorldChunk>{
        self.chunks.get(&IN_coords)
    }
    pub fn getChunkMut(&mut self, IN_coords: Vector2) -> Option<&mut GridWorldChunk>{
        self.chunks.get_mut(&IN_coords)
    }
}
pub struct GridWorldChunk{
    cells: [GridWorldTile; (CHUNK_X * CHUNK_Y) as usize]
}
impl GridWorldChunk{
    pub fn new() -> Self{
        Self{
            cells: [GridWorldTile{mat: 0}; (CHUNK_X * CHUNK_Y) as usize]
        }
    }
}
impl Index<Vector2> for GridWorldChunk{
    type Output = GridWorldTile;

    fn index(&self, index: Vector2) -> &Self::Output {
        // Absolute offsets in the chunks
        let w_XCoord = (index.0 + CHUNK_X) % CHUNK_X;
        let w_YCoord = (index.1 + CHUNK_Y) % CHUNK_Y;
        &self.cells[(w_XCoord + w_YCoord * CHUNK_X) as usize]
    }
}
impl IndexMut<Vector2> for GridWorldChunk{
    fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
        // Absolute offsets in the chunks
        let w_XCoord = (index.0 + CHUNK_X) % CHUNK_X;
        let w_YCoord = (index.1 + CHUNK_Y) % CHUNK_Y;
        &mut self.cells[(w_XCoord + w_YCoord * CHUNK_X) as usize]
    }
}
#[derive(Clone, Copy)]
pub struct GridWorldTile{
    mat: u8 // Just a number for now
}