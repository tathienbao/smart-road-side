use piston_window::*;
use crate::direction::Direction;

pub fn handle_keyboard_event(key: Key) -> Option<Direction> {
    match key {
        Key::Up => Some(Direction::North),
        Key::Down => Some(Direction::South),
        Key::Left => Some(Direction::West),
        Key::Right => Some(Direction::East),
        _ => None,
    }
}

