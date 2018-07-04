use std::collections::HashMap;

use rand::{distributions, thread_rng, Rng};

use tilemap::Tile;

#[derive(Default)]
pub struct EntityMap {
    pub tiles: HashMap<(i32, i32), Tile>,
}

impl EntityMap {
    pub fn new() -> Self {
        EntityMap {
            tiles: HashMap::new(),
        }
    }
}

#[derive(Default)]
pub struct BackgroundMap {
    pub tiles: HashMap<(i32, i32), Tile>,
}

impl BackgroundMap {
    pub fn new() -> Self {
        BackgroundMap {
            tiles: HashMap::new(),
        }
    }

    pub fn generate(&mut self) {
        let mut rng = thread_rng();
        let tile_range = distributions::Uniform::new_inclusive(1, 100);

        for x in 0..128 {
            for y in 0..128 {
                let mut sprite_id = 0;

                match rng.sample(&tile_range) {
                    1 | 2 | 3 => {
                        sprite_id = 1;
                    }
                    4 | 5 | 6 => {
                        sprite_id = 4;
                    }
                    98 | 99 => {
                        sprite_id = 2;
                    }
                    100 => {
                        sprite_id = 3;
                    }
                    _ => {}
                }

                let tile = Tile {
                    sprite_layer: 0,
                    sprite_id,
                };

                self.tiles.insert((x, y), tile);
            }
        }
    }
}
