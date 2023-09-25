use std::collections::VecDeque;
use crate::direction::Direction;

pub struct Car {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

// VecDeque toàn cục để lưu trữ tất cả các xe
pub static mut CARS: VecDeque<Car> = VecDeque::new();

impl Car {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Car { x, y, direction }
    }
}
