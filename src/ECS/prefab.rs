use std::any::Any;

use super::builders::gmObjBuilder;

pub trait gmPrefab: Any{
    fn spawn(&self, IN_builder: gmObjBuilder);
}