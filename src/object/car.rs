use crate::direction::Direction;

pub struct Car {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
    pub prev_x: i32,
    pub prev_y: i32,
    pub direction: Direction,
    pub texture_id: usize,
    pub speed: CarSpeed,
    pub whisker1: Whisker,
    pub whisker2: Whisker,
    pub hit_box: HitBox,
    pub width: i32,
    pub height: i32,
    pub stop_frames: u32,
    pub turn_progress: f64,
    pub angle: f64,
}

impl Car {
    pub fn new(id: usize, x: i32, y: i32, direction: Direction, texture_id: usize) -> Self {
        let hit_box = HitBox { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };
        Car {id, x, y,
            x2: 0,
            y2: 0,
            prev_x: x, prev_y: y, direction, texture_id, speed: CarSpeed::Default, whisker1: Whisker {
            x,
            y,
            dx: 0,
            dy: 0,
        }, whisker2: Whisker {
            x,
            y,
            dx: 0,
            dy: 0,
        }, hit_box, width: 26, height: 60, stop_frames: 0, turn_progress: 0.0, angle: 0.0 }
    }
}

#[derive(Copy, Clone, PartialEq)]
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
    pub x: f64,  // X-coordinate of the top-left corner
    pub y: f64,  // Y-coordinate of the top-left corner
    pub width: f64,  // Width of the hit box
    pub height: f64,  // Height of the hit box
}
