use std::collections::VecDeque;
use crate::direction::Direction;

pub struct Car {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub prev_x: i32,
    pub prev_y: i32,
    pub direction: Direction,
    pub texture_id: usize,
    pub speed: CarSpeed,
    pub whisker: Whisker,
}

// VecDeque (global) to store cars
pub static mut CARS: VecDeque<Car> = VecDeque::new();

impl Car {
    pub fn new(id: usize, x: i32, y: i32, direction: Direction, texture_id: usize) -> Self {
        let whisker = Whisker { x: 0, y: 0, dx: 0, dy: 0 };
        Car {id, x, y, prev_x: x, prev_y: y, direction, texture_id, speed: CarSpeed::Default, whisker}
    }
}

#[derive(Copy, Clone)]
pub enum CarSpeed {
    Stop = 0,
    Slow = 1,
    Default = 5,
}

pub struct Whisker {
    pub x: i32,
    pub y: i32,
    pub dx: i32, // frame to frame
    pub dy: i32, // frame to frame
}
