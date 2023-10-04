use std::collections::VecDeque;
use piston_window::{Texture, G2dTexture, Transformed, TextureSettings, PistonWindow, line, Context, G2d};
use std::fs;
use crate::object::car::{Car, CarSpeed};
use crate::object::direction::Direction;
use crate::logic::logic::{collision_detect, in_intersection, perform_turn, slow_down};

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 1200;
const CAR_WIDTH: f64 = 26.0;
const CAR_HEIGHT: f64 = 49.0;

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
    c: Context,
    g: &mut G2d,
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
        Direction::NorthRight => transform.rot_rad(std::f64::consts::PI/2.0*car.turn_progress + std::f64::consts::PI),
        Direction::SouthRight => transform.rot_rad(std::f64::consts::PI/2.0*car.turn_progress),
        Direction::EastRight => transform.rot_rad(std::f64::consts::PI/2.0*car.turn_progress - std::f64::consts::PI/2.0),
        Direction::WestRight => transform.rot_rad(std::f64::consts::PI/2.0*car.turn_progress + std::f64::consts::PI/2.0),
        Direction::NorthLeft => transform.rot_rad(- std::f64::consts::PI/2.0*car.turn_progress - std::f64::consts::PI),
        Direction::SouthLeft => transform.rot_rad(- std::f64::consts::PI/2.0*car.turn_progress),
        Direction::EastLeft => transform.rot_rad(- std::f64::consts::PI/2.0*car.turn_progress - std::f64::consts::PI/2.0),
        Direction::WestLeft => transform.rot_rad(- std::f64::consts::PI/2.0*car.turn_progress + std::f64::consts::PI/2.0),
    };

    // Draw the car
    piston_window::image(texture, rotated_transform.scale(0.36, 0.36), g);
}

pub fn update_car_position(cars: &mut VecDeque<Car>, current_car_id: usize) {
    let car = &mut cars[current_car_id];

    car.prev_x = car.x;
    car.prev_y = car.y;

    let safe_flag = collision_detect(cars, current_car_id);

    let car = &mut cars[current_car_id];

    // Nếu xe phát hiện va chạm
    if safe_flag {
        // Nếu xe đang chạy nhanh, chuyển sang chạy chậm
        if car.speed == CarSpeed::Default {
            car.speed = CarSpeed::Slow;
        }
        // Nếu xe đang chạy chậm, dừng lại
        else if car.speed == CarSpeed::Slow {
            car.speed = CarSpeed::Stop;
            car.stop_frames = 60; // Dừng lại trong 60 frames
        }
    }
    // Nếu xe không phát hiện va chạm
    else {
        // Nếu xe đang ở trong giao lộ, chạy chậm
        if in_intersection(car) {
            car.speed = CarSpeed::Slow;
        }
        // Nếu xe đang ở ngoài giao lộ, chạy nhanh
        else {
            car.speed = CarSpeed::Default;
        }
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
        Direction::NorthRight => {
            if car.y > WINDOW_HEIGHT / 2 + 200 {
                car.y -= speed;
            } else if car.x > WINDOW_WIDTH / 2 + 200 {
                car.x += speed;
            } else {
                perform_turn(car);
            }
        }
        Direction::SouthRight => {
            if car.y < WINDOW_HEIGHT / 2 - 200 {
                car.y += speed;
            } else if car.x < WINDOW_WIDTH / 2 - 200 {
                car.x -= speed;
            } else {
                perform_turn(car);
            }
        }
        Direction::EastRight => {
            if car.x < WINDOW_WIDTH / 2 - 200 {
                car.x += speed;
            } else if car.y > WINDOW_HEIGHT / 2 + 200 {
                car.y += speed;
            } else {
                perform_turn(car);
            }
        }
        Direction::WestRight => {
            if car.x > WINDOW_WIDTH / 2 + 200 {
                car.x -= speed;
            } else if car.y < WINDOW_HEIGHT / 2 - 200 {
                car.y -= speed;
            } else {
                perform_turn(car);
            }
        }
        Direction::NorthLeft => {
            if car.y > WINDOW_HEIGHT / 2 + 200 {
                car.y -= speed;
            } else if car.x < WINDOW_WIDTH / 2 - 200 {
                car.x -= speed;
            } else {
                perform_turn(car);
            }
        }
        Direction::SouthLeft => {
            if car.y < WINDOW_HEIGHT / 2 - 200 {
                car.y += speed;
            } else if car.x > WINDOW_WIDTH / 2 + 200 {
                car.x += speed;
            } else {
                perform_turn(car);
            }
        }
        Direction::EastLeft => {
            if car.x < WINDOW_WIDTH / 2 - 200 {
                car.x += speed;
            } else if car.y < WINDOW_HEIGHT / 2 - 200 {
                car.y -= speed;
            } else {
                perform_turn(car);
            }
        }
        Direction::WestLeft => {
            if car.x > WINDOW_WIDTH / 2 + 200 {
                car.x -= speed;
            } else if car.y > WINDOW_HEIGHT / 2 + 200 {
                car.y += speed;
            } else {
                perform_turn(car);
            }
        }
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

    // A whisker start from the car's position and ends at the whisker's position
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

pub fn update_hit_box(car: &mut Car) {

    match car.direction{
        Direction::North => {
            car.hit_box.x = car.x as f64 - CAR_WIDTH;
            car.hit_box.y = car.y as f64 - CAR_HEIGHT;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::South => {
            car.hit_box.x = car.x as f64 + CAR_WIDTH;
            car.hit_box.y = car.y as f64 + CAR_HEIGHT;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::East => {
            car.hit_box.x = car.x as f64 + CAR_HEIGHT;
            car.hit_box.y = car.y as f64 - CAR_WIDTH;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::West => {
            car.hit_box.x = car.x as f64 - CAR_HEIGHT;
            car.hit_box.y = car.y as f64 + CAR_WIDTH;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::NorthRight => {
            car.hit_box.x = car.x as f64;
            car.hit_box.y = car.y as f64;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        _ => {
            car.hit_box.x = car.x as f64;
            car.hit_box.y = car.y as f64;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
    }
}

pub fn update_angle(car: &mut Car) {
    car.angle = match car.direction {
        Direction::North => 0.0,
        Direction::East  => std::f64::consts::PI / 2.0,
        Direction::South => std::f64::consts::PI,
        Direction::West  => 3.0 * std::f64::consts::PI / 2.0,
        Direction::NorthRight => std::f64::consts::PI/2.0*car.turn_progress + std::f64::consts::PI,
        Direction::SouthRight => std::f64::consts::PI/2.0*car.turn_progress,
        Direction::EastRight => std::f64::consts::PI/2.0*car.turn_progress - std::f64::consts::PI/2.0,
        Direction::WestRight => std::f64::consts::PI/2.0*car.turn_progress + std::f64::consts::PI/2.0,
        Direction::NorthLeft => - std::f64::consts::PI/2.0*car.turn_progress - std::f64::consts::PI,
        Direction::SouthLeft => - std::f64::consts::PI/2.0*car.turn_progress,
        Direction::EastLeft => - std::f64::consts::PI/2.0*car.turn_progress - std::f64::consts::PI/2.0,
        Direction::WestLeft => - std::f64::consts::PI/2.0*car.turn_progress + std::f64::consts::PI/2.0,
    };
}

