use std::collections::VecDeque;
use piston_window::*;
use rand::Rng;
use crate::direction::Direction;
use crate::direction_renderer::{draw_north_right, draw_east_right, draw_south_right, draw_west_right, draw_north, draw_south, draw_east, draw_west, draw_north_left, draw_south_left, draw_east_left, draw_west_left};
use crate::object::car::Car;
use crate::render::car_renderer::{draw_car, update_car_position, load_all_textures, draw_whisker, update_whisker};
use crate::keyboard::handle_keyboard_event;
use crate::logic::logic::draw_intersection;

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 1200;
const HALF_CAR_WIDTH: i32 = 18;

pub struct RendererManager {
    pub window: PistonWindow,
    pub cars: VecDeque<Car>,
    pub textures: Vec<G2dTexture>,
}

impl RendererManager {
    pub fn new() -> RendererManager {
        let mut window: PistonWindow = WindowSettings::new("Piston Smart Road", [1600, 1200])
            .exit_on_esc(true)
            .build()
            .unwrap();

        window.set_max_fps(120);
        window.set_ups(120);

        let textures = load_all_textures(&mut window);

        Self {
            window,
            cars: VecDeque::new(),
            textures,
        }
    }

    pub fn render(&mut self) {
        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);

                draw_intersection(c, g);

                //draw leading lines
                draw_north_right(c, g);
                draw_east_right(c, g);
                draw_south_right(c, g);
                draw_west_right(c, g);
                draw_north(c, g);
                draw_south(c, g);
                draw_east(c, g);
                draw_west(c, g);
                draw_north_left(c, g);
                draw_south_left(c, g);
                draw_east_left(c, g);
                draw_west_left(c, g);

                for car in &mut self.cars {
                    draw_car(car, &self.textures, c, g);
                    draw_whisker(car, c, g);
                }
            });

            if let Some(Button::Keyboard(key)) = event.press_args() {
                if let Some(direction) = handle_keyboard_event(key) {
                    let texture_id = rand::thread_rng().gen_range(0..self.textures.len());
                    let id = self.cars.len();

                    let (_init_x, _init_y) = match direction {
                        Direction::North => (WINDOW_WIDTH / 2 + 90 + HALF_CAR_WIDTH, WINDOW_HEIGHT),
                        Direction::South => (WINDOW_WIDTH / 2 - 90 - HALF_CAR_WIDTH, 0),
                        Direction::East => (0, WINDOW_HEIGHT / 2 + 90 + HALF_CAR_WIDTH),
                        Direction::West => (WINDOW_WIDTH, WINDOW_HEIGHT / 2 - 90 - HALF_CAR_WIDTH),
                        _ => (0, 0),
                    };

                    let car = Car::new(id ,_init_x, _init_y, direction, texture_id);
                    self.cars.push_back(car);
                }
            }


            // Tạo một Vec để lưu các index cần cập nhật
            let mut indices_to_update: Vec<usize> = Vec::new();
            for (idx, _car) in self.cars.iter().enumerate() {
                indices_to_update.push(idx);
            }

            // Cập nhật các Car dựa trên các index đã lưu
            for idx in indices_to_update {
                update_car_position(&mut self.cars, idx);
            }

            // Update all cars' whiskers
            for car in &mut self.cars {
                update_whisker(car);
            }
        }
    }
}