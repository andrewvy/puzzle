use specs::prelude::*;

use components;
use resources;

#[derive(Default)]
pub struct Plantae {
    pub ticks: u32,
}

impl<'a> System<'a> for Plantae {
    type SystemData = (
        Write<'a, resources::EntityMap>,
        WriteStorage<'a, components::Plantae>,
        ReadStorage<'a, components::Position>,
    );

    fn run(&mut self, (_entity_map, _plantae, _position): Self::SystemData) {
    }
}
