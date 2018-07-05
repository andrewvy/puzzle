use specs;

use components::*;

pub struct World {
    pub specs_world: specs::World,
}

impl World {
    fn register_components(&mut self) {
        self.specs_world.register::<Player>();
        self.specs_world.register::<Position>();
        self.specs_world.register::<Plantae>();
        self.specs_world.register::<Solid>();
        self.specs_world.register::<Sprite>();
        self.specs_world.register::<Movement>();
    }

    pub fn new() -> Self {
        let w = specs::World::new();

        let mut the_world = Self { specs_world: w };

        the_world.register_components();

        the_world
    }
}
