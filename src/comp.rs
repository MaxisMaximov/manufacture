// # Components module
// This is where you can define Components *(data)* for Entities to have
use super::*;

pub struct Velocity2(pub Vector2);
impl Component for Velocity2{
    type STORAGE = HashMapStorage<Self>;

    const ID: &'static str = "Velocity2";
}

pub struct Velocity3(pub Vector3);
impl Component for Velocity3{
    type STORAGE = HashMapStorage<Self>;

    const ID: &'static str = "Velocity3";
}