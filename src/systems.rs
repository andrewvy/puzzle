use specs::prelude::*;

use components::{self, MoveAction};
use resources;
use input::{ControllerState, Buttons};

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

pub struct PlayerMovement;

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (
        Option<Read<'a, ControllerState>>,
        WriteStorage<'a, components::Movement>
    );

    fn run(&mut self, (controller_state, mut movements): Self::SystemData) {
        let controller_state = controller_state.expect("no controller state");


        for movement in (&mut movements).join() {
            if movement.player_owned {
                if controller_state.get_button_pressed(Buttons::Up) {
                    movement.move_queue.push_back(MoveAction::Up);
                } else if controller_state.get_button_pressed(Buttons::Down) {
                    movement.move_queue.push_back(MoveAction::Down);
                } else if controller_state.get_button_pressed(Buttons::Left) {
                    movement.move_queue.push_back(MoveAction::Left);
                } else if controller_state.get_button_pressed(Buttons::Right) {
                    movement.move_queue.push_back(MoveAction::Right);
                }
            }
        }
    }
}

pub struct ProcessMovement;

impl<'a> System<'a> for ProcessMovement {
    type SystemData = (
        WriteStorage<'a, components::Movement>,
        WriteStorage<'a, components::Position>,
    );

    fn run(&mut self, (mut movements, mut positions): Self::SystemData) {
        for (movement, position) in (&mut movements, &mut positions).join() {
            let (target_x, target_y) = match movement.move_queue.front() {
                Some(&MoveAction::Up) => (position.x, position.y - 1),
                Some(&MoveAction::Down) => (position.x, position.y + 1),
                Some(&MoveAction::Left) => (position.x - 1, position.y),
                Some(&MoveAction::Right) => (position.x + 1, position.y),
                _ => continue,
            };

            movement.move_queue.pop_front();

            position.x = target_x;
            position.y = target_y;
        }
    }
}
