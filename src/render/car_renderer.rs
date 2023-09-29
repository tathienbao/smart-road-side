use piston_window::{Texture, G2dTexture, Transformed, TextureSettings, PistonWindow};
use std::fs;
use crate::object::car::{Car, CarSpeed};
use crate::object::direction::Direction;
use crate::logic::logic::in_intersection;

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


    piston_window::image(texture, rotated_transform.scale(0.5, 0.5), g);
}

pub fn update_car_position(car: &mut Car) {
    if in_intersection(car) {
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
