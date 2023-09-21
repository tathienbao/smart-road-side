use crate::render::direction_renderer::get_waypoints_for_direction;
use crate::direction::Direction;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Car {
    x: i32,
    y: i32,
    speed: f32,
    direction: Direction,
    current_waypoint_index: usize,
    waypoints: Vec<Point>,
}

impl Car {
    pub fn new(position: (i32, i32), speed: f32, direction: Direction) -> Self {
        let waypoints = get_waypoints_for_direction(&direction);

        Self {
            x: position.0,
            y: position.1,
            speed,
            direction,
            current_waypoint_index: 0,
            waypoints,
        }
    }

    pub fn update(&mut self) {
        if self.current_waypoint_index < self.waypoints.len() {
            let next_point = &self.waypoints[self.current_waypoint_index];

            // Simplified approach: Assuming car moves 1 pixel per update
            if self.x < next_point.x() {
                self.x += 1;
            } else if self.x > next_point.x() {
                self.x -= 1;
            }

            if self.y < next_point.y() {
                self.y += 1;
            } else if self.y > next_point.y() {
                self.y -= 1;
            }

            // When we reach the waypoint, increment the index
            if self.reached_waypoint(next_point) {
                self.current_waypoint_index += 1;
            }
        }
    }

    fn reached_waypoint(&self, waypoint: &Point) -> bool {
        self.x == waypoint.x() && self.y == waypoint.y()
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255)); // Setting the car's color to blue
        canvas.fill_rect(sdl2::rect::Rect::new(self.x, self.y, 20, 10)).unwrap();
    }
}
