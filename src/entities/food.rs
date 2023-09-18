use rand::prelude::*;
use crate::geometry::*;
use crate::entities::player::Player;

pub struct Food {
    pub location: Point
}

impl Food {
    pub fn new() -> Food {
        Food {
            location: Point(3, 3),
        }
    }

    pub fn update(&mut self, player: &Player) {
        while player.contains(&self.location) {
            let mut rng = thread_rng();
            let food_x = rng.gen_range(1..GRID_X_SIZE);
            let food_y = rng.gen_range(1..GRID_Y_SIZE);

            self.location = Point(food_x, food_y);
        }
    }
}
