// # Sprites module
// This is where Manufacture sprites are stored
use super::*;

use manufacture_core::types::ASCIIImage;

pub fn init(world: &mut World){
    let mut sprite_registry = world.fetch_res_mut::<manufacture_core::resources::CMDSpriteRegistry>();

    sprite_registry.register("Player1".to_owned(), player());
    sprite_registry.register("Crafter".to_owned(), crafter());
}

fn player() -> ASCIIImage{
    ASCIIImage{
        size_x: 1,
        size_y: 1,
        data: Box::new(
            [('P', (255, 255, 255), (0, 128, 255))]
        ),
    }
}

fn crafter() -> ASCIIImage{
    ASCIIImage{
        size_x: 2,
        size_y: 2,
        data: Box::new(
            [
                ('C', (255, 255, 255), (255, 170, 0)), ('R', (255, 255, 255), (255, 170, 0)),
                ('F', (255, 255, 255), (255, 170, 0)), ('T', (255, 255, 255), (255, 170, 0)),
            ]
        ),
    }
}