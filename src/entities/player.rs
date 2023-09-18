use std::collections::VecDeque;
use crate::geometry::*;
use crate::entities::food::Food;

pub struct Player {
    pub body: VecDeque<Point>,
    direction: Direction,
    alive: bool,
    pub score: u32
}

impl Player {
    pub fn new() -> Player {
        Player {
            body: VecDeque::from([Point(1,1), Point(2, 1), Point(3, 1)]),
            direction: Direction::Right,
            alive: true,
            score: 3
        }
    }

    pub fn update(&mut self, food:&Food, input_direction: Direction) -> () {
        if input_direction != self.direction.opposite() {
            self.direction = input_direction.clone();
        }

       let head: Point = self.body.back().unwrap().clone();
       let next_move = head + self.direction.unit();

       let self_intersects = self.contains(&next_move);
       let collides_with_wall = !in_bounds(&next_move);

       if self_intersects || collides_with_wall {
           self.alive = false;
           return
       }

       self.body.push_back(next_move);

       let food_eaten = self.contains(&food.location);
       if !food_eaten {
           self.body.pop_front();
       }
       else {
           self.score += 1
       }
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.body.contains(point)
    }

    pub fn is_dead(&self) -> bool { !&self.alive } 
}

