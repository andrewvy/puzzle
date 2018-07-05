use specs::prelude::*;

use components::*;
use world::World;

pub fn create_player(
    world: &mut World,
    x: i32,
    y: i32,
) -> Entity {
    world
        .specs_world
        .create_entity()
        .with(Position::new(x, y))
        .with(Movement::new(true))
        .with(Sprite::new(1, 5))
        .build()
}
