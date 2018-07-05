use std::collections::VecDeque;
use std::time::Instant;

use specs::prelude::*;

use tilemap::Tile;

#[derive(Debug, Default)]
pub struct Player {}

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

pub struct Sprite {
    pub tile: Tile,
}

impl Sprite {
    pub fn new(sprite_layer: i32, sprite_id: i32) -> Sprite {
        Sprite {
            tile: Tile {
                sprite_layer,
                sprite_id,
            }
        }
    }
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Solid;

impl Component for Solid {
    type Storage = NullStorage<Self>;
}

#[derive(Debug)]
pub struct Plantae {
}

impl Plantae {
    pub fn new() -> Self {
        Plantae {}
    }
}

impl Component for Plantae {
    type Storage = VecStorage<Self>;
}

pub enum MoveAction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Movement {
    pub player_owned: bool,
    pub move_queue: VecDeque<MoveAction>,
}

impl Movement {
    pub fn new(player_owned: bool) -> Self {
        Movement {
            player_owned,
            move_queue: VecDeque::new(),
        }
    }
}

impl Component for Movement {
    type Storage = VecStorage<Self>;
}

pub struct AnimationEventPosition {
    pub offset: (i32, i32),
    pub start: Instant,
    pub end: Instant,
}

pub enum AnimationEvent {
    Position(AnimationEventPosition),
}

pub struct Animation {
    pub animation_queue: VecDeque<AnimationEvent>,
}

impl Animation {
    pub fn new() -> Self {
        Animation {
            animation_queue: VecDeque::new(),
        }
    }
}

impl Component for Animation {
    type Storage = VecStorage<Self>;
}
