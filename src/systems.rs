// # Systems module
// This is where you can define Systems that will be responsible for running your app
use super::*;
use comp::*;

pub struct Move2D;
impl System for Move2D{
    type Data<'a> = Query<(&'a Velocity2, &'a mut Transform2D), ()>;

    const ID: &'static str = "Move2D";

    fn new() -> Self {
        Self
    }

    fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {
        for (vel, pos) in data.iter_mut(){
            pos.loc += vel.0;
        }
    }
}