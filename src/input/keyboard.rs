use piston_window::*;
use crate::direction::Direction;

use rand::Rng;

pub fn handle_keyboard_event(key: Key) -> Option<Direction> {
    let mut rng = rand::thread_rng();
    match key {
        Key::Up => {
            match rng.gen_range(0..3) {
                0 => Some(Direction::North),
                1 => Some(Direction::NorthRight),
                2 => Some(Direction::NorthLeft),
                _ => None,
            }
        },
        Key::Down => {
            match rng.gen_range(0..3) {
                0 => Some(Direction::South),
                1 => Some(Direction::SouthRight),
                2 => Some(Direction::SouthLeft),
                _ => None,
            }
        },
        Key::Left => {
            match rng.gen_range(0..3) {
                0 => Some(Direction::West),
                1 => Some(Direction::WestRight),
                2 => Some(Direction::WestLeft),
                _ => None,
            }
        },
        Key::Right => {
            match rng.gen_range(0..3) {
                0 => Some(Direction::East),
                1 => Some(Direction::EastRight),
                2 => Some(Direction::EastLeft),
                _ => None,
            }
        },
        Key::R => {
            let random_choice = rng.gen_range(0..12);
            match random_choice {
                0 => Some(Direction::North),
                1 => Some(Direction::South),
                2 => Some(Direction::East),
                3 => Some(Direction::West),
                4 => Some(Direction::NorthRight),
                5 => Some(Direction::SouthRight),
                6 => Some(Direction::EastRight),
                7 => Some(Direction::WestRight),
                8 => Some(Direction::NorthLeft),
                9 => Some(Direction::SouthLeft),
                10 => Some(Direction::EastLeft),
                11 => Some(Direction::WestLeft),
                _ => None,
            }
        },
        _ => None,
    }
}

