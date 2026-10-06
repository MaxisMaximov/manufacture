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

pub struct DrawScheduler;
impl System for DrawScheduler{
    type Data<'a> = (Query<(&'a Transform2D, &'a CMDSprite), ()>, &'a mut CMDRenderQueue, &'a CMDCamera);

    const ID: &'static str = "DrawScheduler";

    const DEPENDS: &'static [&'static str] = &[CMDRenderer::ID];
    const RUNORD: &'static [RunOrder] = &[RunOrder::Before(CMDRenderer::ID)];
    const TYPE: SystemType = SystemType::Postprocessor;

    fn new() -> Self {
        Self
    }

    fn execute(&mut self, data: Request<'_, Self::Data<'_>>) {
        let (
            query,
            mut draw_queue,
            camera
        ) = data.into_raw();

        for (loc, spr) in query.iter(){
            let loc_diff = camera.pos - loc.loc;
            draw_queue.push(
                CMDRenderCommand::DrawSprite{
                    pos: (loc_diff.x, loc_diff.y),
                    z: 1.0,
                    sprite_id: spr.id.clone()
                }
            );
        }
    }
}