use std::ops::Add;

pub const GRID_X_SIZE: i32 = 40;
pub const GRID_Y_SIZE: i32 = 30;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(pub i32, pub i32);
impl Add<Point> for Point {
    type Output = Self;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction { Up, Left, Right, Down }

pub fn opposite(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left
    }
} 

pub fn in_bounds(point: &Point) -> bool {
    let Point(x, y) = point;
    &0 < x && x < &GRID_X_SIZE && &0 < y && y < &GRID_Y_SIZE
}
