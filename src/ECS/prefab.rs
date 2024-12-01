use super::builders::gmObjBuilder;

pub trait gmPrefab{
    fn spawn(self, IN_builder: gmObjBuilder);
}