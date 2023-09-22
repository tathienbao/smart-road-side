use crate::direction::Direction;
use crate::render::car_renderer;

pub struct Car {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

impl Car {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Car { x, y, direction }
    }

    pub fn update_position(&mut self) {
        car_renderer::update_car_position(self);
    }
}
