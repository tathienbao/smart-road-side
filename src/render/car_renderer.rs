use piston_window::{Texture, G2dTexture, TextureSettings, Flip};
use std::fs;
use image::ImageFormat;
use std::path::Path;
use crate::object::car::Car;
use crate::object::direction::Direction;
use rand::Rng;

pub fn load_all_textures(factory: &mut G2dTexture) -> Vec<G2dTexture> {
    let mut textures = Vec::new();
    let paths = fs::read_dir("./assets").unwrap();

    for path in paths {
        let path_str = path.unwrap().path();
        if path_str.extension().unwrap_or_default() == "png" {
            let texture: G2dTexture = Texture::from_path(
                factory,
                &path_str,
                Flip::None,
                &TextureSettings::new(),
            ).unwrap();
            textures.push(texture);
        }
    }

    textures
}

pub fn choose_random_texture<'a>(textures: &'a [G2dTexture]) -> &'a G2dTexture {
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
