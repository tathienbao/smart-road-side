use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::object::Car;
use crate::direction::Direction;

pub fn draw_car(car: &mut Car, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    match car.direction {
        Direction::North | Direction::South => {
            canvas.fill_rect(sdl2::rect::Rect::new(car.x, car.y, 20, 40)).unwrap();
        }
        Direction::East | Direction::West => {
            canvas.fill_rect(sdl2::rect::Rect::new(car.x, car.y, 40, 20)).unwrap();
        }
        _ => {}
    };
}

// This function will allow the car to move based on its direction
pub fn update_car_position(car: &mut Car) {
    match car.direction {
        Direction::North => {
            car.y -= 5;
        }
        Direction::South => {
            car.y += 5;
        }
        Direction::East => {
            car.x += 5;
        }
        Direction::West => {
            car.x -= 5;
        }
        _ => {}
    }
}
