use std::collections::VecDeque;
use piston_window::{Texture, G2dTexture, Transformed, TextureSettings, PistonWindow, line, Context, G2d};
use std::fs;
use crate::object::car::{Car, CarSpeed};
use crate::object::direction::Direction;
use crate::logic::logic::{collision_detect, in_intersection, make_second_point, perform_turn, slow_down};

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 1200;
pub const CAR_WIDTH: f64 = 26.0;
const CAR_HEIGHT: f64 = 60.0;

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

    // safety distance
    if safe_flag {
        slow_down(car);
        if car.speed == CarSpeed::Stop {
            car.stop_frames = 60;
        }
    } else if in_intersection(car) {
        if safe_flag {
            car.speed = CarSpeed::Stop;
        } else {
            car.speed = CarSpeed::Slow;
        }
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

pub fn update_second_point(car: &mut Car, angle: f64, width: i32) {
    let (new_x, new_y) = match car.direction {
        Direction::North => make_second_point((car.x, car.y), angle, -width),
        Direction::South => make_second_point((car.x, car.y), angle, -width),
        Direction::East => make_second_point((car.x, car.y), angle, -width),
        Direction::West => make_second_point((car.x, car.y), angle, -width),
        Direction::NorthRight => make_second_point((car.x, car.y), angle, width),
        Direction::SouthRight => make_second_point((car.x, car.y), angle, width),
        Direction::EastRight => make_second_point((car.x, car.y), angle, width),
        Direction::WestRight => make_second_point((car.x, car.y), angle, width),
        Direction::NorthLeft => make_second_point((car.x, car.y), angle, width),
        Direction::SouthLeft => make_second_point((car.x, car.y), angle, width),
        Direction::EastLeft => make_second_point((car.x, car.y), angle, width),
        Direction::WestLeft => make_second_point((car.x, car.y), angle, width),
    };
    car.x2 = new_x;
    car.y2 = new_y;
}

pub fn draw_whisker(
    car: &Car,
    c: Context,
    g: &mut G2d,
) {
    let blue: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

    // Draw the first whisker
    line(
        blue,
        1.0,
        [car.x as f64, car.y as f64, car.whisker1.x as f64, car.whisker1.y as f64],
        c.transform,
        g,
    );

    // Draw the second whisker
    line(
        blue,
        1.0,
        [car.x2 as f64, car.y2 as f64, car.whisker2.x as f64, car.whisker2.y as f64],
        c.transform,
        g,
    );
}

pub fn update_whisker(car: &mut Car) {
    let dx = car.x - car.prev_x;
    let dy = car.y - car.prev_y;

    // Update the previous position
    car.prev_x = car.x;
    car.prev_y = car.y;

    // Update the first whisker's position
    car.whisker1.x = car.x + dx * 50;  // Use 50 as the length of the whisker
    car.whisker1.y = car.y + dy * 50;

    // Update the second whisker's position
    // Assuming the second whisker's origin is offset by 10 units in x and y direction
    car.whisker2.x = car.x2 + dx * 50;
    car.whisker2.y = car.y2 + dy * 50;
}


pub fn update_hit_box(car: &mut Car) {
    match car.direction {
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
        Direction::SouthRight => {
            car.hit_box.x = car.x as f64;
            car.hit_box.y = car.y as f64;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::EastRight => {
            car.hit_box.x = car.x as f64;
            car.hit_box.y = car.y as f64;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::WestRight => {
            car.hit_box.x = car.x as f64;
            car.hit_box.y = car.y as f64;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::NorthLeft => {
            car.hit_box.x = car.x as f64;
            car.hit_box.y = car.y as f64;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::SouthLeft => {
            car.hit_box.x = car.x as f64;
            car.hit_box.y = car.y as f64;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::EastLeft => {
            car.hit_box.x = car.x as f64;
            car.hit_box.y = car.y as f64;
            car.hit_box.width = CAR_WIDTH;
            car.hit_box.height = CAR_HEIGHT;
        }
        Direction::WestLeft => {
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

