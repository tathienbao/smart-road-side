use std::collections::VecDeque;
use piston_window::{Texture, G2dTexture, Transformed, TextureSettings, PistonWindow, line, Context, G2d};
use std::fs;
use crate::object::car::{Car, CarSpeed};
use crate::object::direction::Direction;
use crate::logic::logic::{in_intersection, should_stop};

pub fn load_all_textures(window: &mut PistonWindow) -> Vec<G2dTexture> {
    let mut textures = Vec::new();
    let paths = fs::read_dir("./assets").expect("CAN'T ENTRY ASSETS FOLDER");

    for path in paths {
        let path_str = path.expect("CAN'T BUFF").path();
        if path_str.extension().unwrap_or_default() == "png" {
            let img = image::open(&path_str).unwrap().to_rgba8();
            let texture = Texture::from_image(
                &mut window.create_texture_context(),
                &img,
                &TextureSettings::new()
            ).unwrap();
            textures.push(texture);
        }
    }

    textures
}

pub fn draw_car(
    car: &Car,
    textures: &[G2dTexture],
    c: piston_window::Context,
    g: &mut piston_window::G2d,
) {
    let texture = &textures[car.texture_id];

    // Translate the car to its position
    let transform = c.transform.trans(car.x as f64, car.y as f64);

    // Rotate the car according to its direction
    let rotated_transform = match car.direction {
        Direction::North => transform.rot_rad(std::f64::consts::PI),  // No rotation
        Direction::South => transform.rot_rad(0.0),  // 180 degrees
        Direction::East => transform.rot_rad(-std::f64::consts::PI / 2.0),  // 90 degrees counter-clockwise
        Direction::West => transform.rot_rad(std::f64::consts::PI / 2.0),  // 90 degrees clockwise
        _ => transform.rot_rad(0.0),  // Default to no rotation
    };

    // Draw the car
    piston_window::image(texture, rotated_transform.scale(0.5, 0.5), g);
}

pub fn update_car_position(cars: &mut VecDeque<Car>, current_car_id: usize) {
    let car = &mut cars[current_car_id];

    car.prev_x = car.x;
    car.prev_y = car.y;

    // Check to avoid borrowing error
    let should_stop_flag = should_stop(cars, current_car_id);

    let car = &mut cars[current_car_id];
    if should_stop_flag {
        car.speed = CarSpeed::Stop;
    } else if in_intersection(car) {
        car.speed = CarSpeed::Slow;
    } else {
        car.speed = CarSpeed::Default;
    }

    let speed = car.speed as i32;

    match car.direction {
        Direction::North => {
            car.y -= speed;
        }
        Direction::South => {
            car.y += speed;
        }
        Direction::East => {
            car.x += speed;
        }
        Direction::West => {
            car.x -= speed;
        }
        _ => {}
    }
}

pub fn draw_whisker(
    car: &Car,
    c: Context,
    g: &mut G2d,
) {
    let car_x = car.x as f64;
    let car_y = car.y as f64;
    let whisker_x = car.whisker.x as f64;
    let whisker_y = car.whisker.y as f64;

    let blue: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

    // draw a line from the car to the whisker
    line(blue, 1.0, [car_x, car_y, whisker_x, whisker_y], c.transform, g);
}


pub fn update_whisker(car: &mut Car) {
    // Calculate dx and dy between the current position and the previous position
    let dx = car.x - car.prev_x;
    let dy = car.y - car.prev_y;

    // Update the previous position
    car.prev_x = car.x;
    car.prev_y = car.y;

    // Update the whisker's position based on dx and dy
    car.whisker.x = car.x + dx * 50;  // Use 50 as the length of the whisker
    car.whisker.y = car.y + dy * 50;
}

