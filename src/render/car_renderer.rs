use piston_window::{Texture, G2dTexture, Transformed, TextureSettings, PistonWindow};
use std::fs;
use crate::object::car::Car;
use crate::object::direction::Direction;
use rand::Rng;

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
    let index = rng.gen_range(1..textures.len());
    &textures[index]
}

pub fn draw_car(
    car: &Car,
    textures: &[G2dTexture],
    c: piston_window::Context,
    g: &mut piston_window::G2d,
) {
    let texture = choose_random_texture(textures);

    let transform = c.transform.trans(car.x as f64, car.y as f64);

    let rotated_transform = match car.direction {
        Direction::North => transform.rot_rad(0.0),  // No rotation
        Direction::South => transform.rot_rad(std::f64::consts::PI),  // 180 degrees
        Direction::East => transform.rot_rad(-std::f64::consts::PI / 2.0),  // 90 degrees counter-clockwise
        Direction::West => transform.rot_rad(std::f64::consts::PI / 2.0),  // 90 degrees clockwise
        _ => transform.rot_rad(0.0),  // Default to no rotation
    };

    piston_window::image(texture, rotated_transform, g);
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
