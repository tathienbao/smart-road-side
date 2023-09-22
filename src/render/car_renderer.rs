use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use crate::direction::Direction;

const CAR_WIDTH: i32 = 20;
const CAR_HEIGHT: i32 = 40;
const CAR_COLOR: (u8, u8, u8) = (0, 0, 255);  // Blue

pub fn draw_car(direction: Direction, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color((CAR_COLOR.0, CAR_COLOR.1, CAR_COLOR.2));

    let rect = match direction {
        Direction::North => Rect::new(400, 600, CAR_WIDTH as u32, CAR_HEIGHT as u32),
        Direction::South => Rect::new(400, 0, CAR_WIDTH as u32, CAR_HEIGHT as u32),
        Direction::East => Rect::new(0, 300, CAR_HEIGHT as u32, CAR_WIDTH as u32),
        Direction::West => Rect::new(800, 300, CAR_HEIGHT as u32, CAR_WIDTH as u32),
        _ => panic!("Unexpected direction"),
    };

    canvas.fill_rect(rect).unwrap();
}
