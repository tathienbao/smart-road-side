use piston_window::{Texture, G2dTexture, Transformed, TextureSettings, PistonWindow};
use std::fs;
use crate::object::car::Car;
use crate::object::direction::Direction;
use rand::Rng;

const WINDOW_WIDTH: f64 = 1600.0;
const WINDOW_HEIGHT: f64 = 1200.0;

const HALF_CAR_WIDTH: f64 = 17.0;

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

pub fn choose_random_texture(textures: & [G2dTexture]) -> &G2dTexture {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..textures.len());
    &textures[index]
}

pub fn draw_car(
    car: &Car,
    textures: &[G2dTexture],
    c: piston_window::Context,
    g: &mut piston_window::G2d,
) {
    let texture = &textures[car.texture_id];

    // Translate the car to its position
    let transform = c.transform.trans(
        match car.direction {
            Direction::North => car.x as f64 + WINDOW_WIDTH/2.0 + 90.0 + HALF_CAR_WIDTH,
            Direction::South => car.x as f64 + WINDOW_WIDTH/2.0 - 90.0 - HALF_CAR_WIDTH,
            Direction::East => car.x as f64,
            Direction::West => car.x as f64 + WINDOW_WIDTH,
            _ => car.x as f64,
        },
        match car.direction {
            Direction::North => car.y as f64 + WINDOW_HEIGHT,
            Direction::South => car.y as f64,
            Direction::East => car.y as f64 + WINDOW_HEIGHT/2.0 + 90.0 + HALF_CAR_WIDTH,
            Direction::West => car.y as f64 + WINDOW_HEIGHT/2.0 - 90.0 - HALF_CAR_WIDTH,
            _ => car.y as f64,
        },
    );

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
    match car.direction {
        Direction::North => {
            car.y -= 1;
        }
        Direction::South => {
            car.y += 1;
        }
        Direction::East => {
            car.x += 1;
        }
        Direction::West => {
            car.x -= 1;
        }
        _ => {}
    }
}
