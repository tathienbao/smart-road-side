use piston_window::*;
use rand::Rng;
use crate::direction::Direction;
use crate::direction_renderer::{draw_north_right, draw_east_right, draw_south_right, draw_west_right, draw_north, draw_south, draw_east, draw_west, draw_north_left, draw_south_left, draw_east_left, draw_west_left};
use crate::object::car::Car;
use crate::render::car_renderer::{draw_car, update_car_position, load_all_textures};
use crate::keyboard::handle_keyboard_event;

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 1200;

pub struct RendererManager {
    pub window: PistonWindow,
    pub cars: Vec<Car>,
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
            cars: Vec::new(),
            textures,
        }
    }

    pub fn render(&mut self) {
        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);

                //draw leading line
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
                }
            });

            if let Some(Button::Keyboard(key)) = event.press_args() {
                if let Some(direction) = handle_keyboard_event(key) {
                    let texture_id = rand::thread_rng().gen_range(0..self.textures.len());

                    let car = Car::new(0, 0, direction, texture_id);
                    self.cars.push(car);
                }
            }

            // Update all cars' positions
            for car in &mut self.cars {
                update_car_position(car);
            }
        }
    }
}