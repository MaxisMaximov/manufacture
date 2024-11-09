use super::*;

use comp::*;
use storage::*;
use vars::*;
use world::*;

pub struct gmObjBuilder<'a>{
    pub gmObjID: gmID,
    pub worldRef: &'a mut gmWorld
}
impl gmObjBuilder<'_>{
    pub fn addComp<T>(self, IN_comp: T) -> Self where T:gmComp{
        // I gotta deal with this
        self.worldRef.fetchMut::<T>().inner.insert(self.gmObjID, IN_comp);
        self
    }
}