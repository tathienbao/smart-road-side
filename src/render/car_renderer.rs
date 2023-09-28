use std::fs;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::image::LoadTexture;
use sdl2::video::Window;
use sdl2::rect::Rect;
use crate::object::car::Car;
use crate::object::direction::Direction;
use rand::Rng;

pub fn load_all_textures(
    texture_creator: &TextureCreator<sdl2::video::WindowContext>,
) -> Vec<Texture> {
    let mut textures = Vec::new();
    let paths = fs::read_dir("./assets").unwrap();

    for path in paths {
        let path_str = path.unwrap().path().display().to_string();
        if path_str.ends_with(".png") {
            let texture = texture_creator.load_texture(path_str).unwrap();
            textures.push(texture);
        }
    }

    textures
}

pub fn choose_random_texture<'a>(textures: &'a Vec<Texture<'a>>) -> &'a Texture<'a> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..textures.len());
    &textures[index]
}

pub fn draw_car(
    car: &Car,
    canvas: &mut Canvas<Window>,
    all_textures: &Vec<Texture>,
) {
    let texture = choose_random_texture(all_textures);
    let src = Rect::new(0, 0, 75, 130);

    let dst = match car.direction {
        Direction::North => Rect::new(car.x, car.y, 75, 130),
        Direction::South => Rect::new(car.x, car.y, 75, 130),
        Direction::East => Rect::new(car.x, car.y, 130, 75),
        Direction::West => Rect::new(car.x, car.y, 130, 75),
        _ => Rect::new(car.x, car.y, 75, 130),
    };

    canvas.copy(&texture, src, dst).expect("Failed to copy texture");
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
