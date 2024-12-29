use std::ops::{Index, IndexMut, Deref, DerefMut};

use super::*;

use types::*;

mod event;
mod world;
mod ui;

pub use event::*;
pub use world::*;
pub use ui::*;

pub struct res_PInput{
    res: KeyEvent
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
impl Deref for res_PInput{
    type Target = KeyEvent;

    fn deref(&self) -> &Self::Target {
        &self.res
    }
}
impl DerefMut for res_PInput{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.res
    }
}

pub struct res_DeltaT{
    res: Duration
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
impl Deref for res_DeltaT{
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.res
    }
}
impl DerefMut for res_DeltaT{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.res
    }
}

pub struct res_PID{
    res: HashMap<gmID, gmID> // PID, gmObjID
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
impl Deref for res_PID{
    type Target = HashMap<gmID, gmID>;

    fn deref(&self) -> &Self::Target {
        &self.res
    }
}
impl DerefMut for res_PID{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.res
    }
}

pub struct res_UIData{
    res: HashMap<&'static str, String>
}
impl gmRes for res_UIData{
    fn new() -> Self {
        Self{
            res: HashMap::new()
        }
    }

    fn RES_ID() -> &'static str {
        "res_UIData"
    }
}
impl Deref for res_UIData{
    type Target = HashMap<&'static str, String>;

    fn deref(&self) -> &Self::Target {
        &self.res
    }
}
impl DerefMut for res_UIData{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.res
    }
}

pub struct res_LoadedChunks{
    res: HashMap<Vector2, gmID>
}
impl gmRes for res_LoadedChunks{
    fn new() -> Self {
        Self{
            res: HashMap::new(),
        }
    }

    fn RES_ID() -> &'static str {
        "res_LoadedChunks"
    }
}
impl Deref for res_LoadedChunks{
    type Target = HashMap<Vector2, gmID>;

    fn deref(&self) -> &Self::Target {
        &self.res
    }
}
impl DerefMut for res_LoadedChunks{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.res
    }
}