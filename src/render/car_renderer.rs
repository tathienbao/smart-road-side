use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::car::Car;
use crate::direction::Direction;
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::rect::Rect;
use crate::Car;

pub fn draw_car(car: &mut Car, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/car.png").unwrap();
    let src = Rect::new(0, 0, 64, 64);

    let dst = match car.direction {
        Direction::North => Rect::new(car.x, car.y, 64, 64),
        Direction::South => Rect::new(car.x, car.y, 64, 64),
        Direction::East => Rect::new(car.x, car.y, 64, 64),
        Direction::West => Rect::new(car.x, car.y, 64, 64),
        _ => Rect::new(car.x, car.y, 64, 64),
    };

    canvas.copy(&texture, src, dst).expect("Failed to copy texture");
}

pub fn update_car_position(car: &mut Car) {
    match car.direction {
        Direction::North => {
            car.y -= 2;
        }
        Direction::South => {
            car.y += 2;
        }
        Direction::East => {
            car.x += 2;
        }
        Direction::West => {
            car.x -= 2;
        }
        _ => {}
    }
}
