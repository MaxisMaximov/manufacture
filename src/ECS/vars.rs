use super::*;

use misc::gmGenIndex;

pub type gmWorld_COMPMAP = HashMap<&'static str, Box<Rc<dyn Any>>>;
pub type gmWorld_RESMAP = HashMap<&'static str, Box<dyn Any>>;
pub type gmObj = gmGenIndex<()>;
pub type gmID = u16;
pub type gmGen = u16; // There is no way you can even remotely get to 32kth generation -- Consider it a gift