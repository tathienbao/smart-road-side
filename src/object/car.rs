use std::collections::VecDeque;
use crate::direction::Direction;

pub struct Car {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub texture_id: usize,
    pub speed: CarSpeed,
}

// VecDeque (global) to store cars
pub static mut CARS: VecDeque<Car> = VecDeque::new();

impl Car {
    pub fn new(x: i32, y: i32, direction: Direction, texture_id: usize) -> Self {
        Car { x, y, direction, texture_id, speed: CarSpeed::Default }
    }
}

#[derive(Copy, Clone)]
pub enum CarSpeed {
    Stop = 0,
    Slow = 1,
    Default = 5,
}
