use sdl2::event::Event;
use crate::direction::Direction;

pub fn handle_keyboard_event(event: &Event) -> Option<Direction> {
    match event {
        Event::KeyDown { keycode, .. } => {
            match keycode {
                Some(sdl2::keyboard::Keycode::Up) => Some(Direction::North),
                Some(sdl2::keyboard::Keycode::Down) => Some(Direction::South),
                Some(sdl2::keyboard::Keycode::Left) => Some(Direction::West),
                Some(sdl2::keyboard::Keycode::Right) => Some(Direction::East),
                _ => None,
            }
        },
        _ => None,
    }
}
