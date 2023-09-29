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
    pub(crate) hit_box: HitBox,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

// VecDeque (global) to store cars
pub static mut CARS: VecDeque<Car> = VecDeque::new();

impl Car {
    pub fn new(id: usize, x: i32, y: i32, direction: Direction, texture_id: usize) -> Self {
        let whisker = Whisker { x: 0, y: 0, dx: 0, dy: 0 };
        let hit_box = HitBox { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };
        Car {id, x, y, prev_x: x, prev_y: y, direction, texture_id, speed: CarSpeed::Default, whisker, hit_box, width: 37, height: 37 }
    }

    pub fn update_hit_box(&mut self) {
        self.hit_box.x = self.x as f64;
        self.hit_box.y = self.y as f64;
        self.hit_box.width = self.width as f64;
        self.hit_box.height = self.height as f64;
    }
}

#[derive(Copy, Clone)]
pub enum CarSpeed {
    Stop = 0,
    Slow = 2,
    Default = 5,
}

pub struct Whisker {
    pub x: i32,
    pub y: i32,
    pub dx: i32, // new frame - old fame (delta x)
    pub dy: i32, // new frame - old fame (delta y)
}

pub struct HitBox {
    pub(crate) x: f64,  // X-coordinate of the top-left corner
    pub(crate) y: f64,  // Y-coordinate of the top-left corner
    pub(crate) width: f64,  // Width of the hit box
    pub(crate) height: f64,  // Height of the hit box
}