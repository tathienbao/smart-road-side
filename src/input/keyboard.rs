use piston_window::*;
use crate::direction::Direction;

use rand::Rng;

pub fn handle_keyboard_event(key: Key) -> Option<Direction> {
    let mut rng = rand::thread_rng();
    match key {
        Key::Up => {
            match rng.gen_range(0..2) {  // Adjust the range based on how many directions you want to add in the future
                0 => Some(Direction::North),
                1 => Some(Direction::NorthRight),
                // Add more directions here in the future
                _ => None,
            }
        },
        Key::Down => Some(Direction::South),
        Key::Left => Some(Direction::West),
        Key::Right => Some(Direction::East),
        Key::R => {
            let random_choice = rng.gen_range(0..4);
            match random_choice {
                0 => Some(Direction::North),
                1 => Some(Direction::South),
                2 => Some(Direction::East),
                3 => Some(Direction::West),
                _ => None,
            }
        },
        _ => None,
    }
}

