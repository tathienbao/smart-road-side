use std::collections::VecDeque;
use piston_window::*;
use rand::Rng;
use crate::direction::Direction;
use crate::direction_renderer::{draw_north_right, draw_east_right, draw_south_right, draw_west_right, draw_north, draw_south, draw_east, draw_west, draw_north_left, draw_south_left, draw_east_left, draw_west_left};
use crate::object::car::Car;
use crate::render::car_renderer::{draw_car, update_car_position, load_all_textures, draw_whisker, update_whisker, update_hit_box};
use crate::keyboard::handle_keyboard_event;
use crate::logic::logic::{draw_intersection, draw_rectangle_edges, safe_spawning};

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 1200;

pub struct RendererManager {
    pub window: PistonWindow,
    pub cars: VecDeque<Car>,
    pub textures: Vec<G2dTexture>,
    pub show_hit_box_and_whisker: bool,
}

impl RendererManager {
    pub fn new() -> RendererManager {
        let mut window: PistonWindow = WindowSettings::new("Piston Smart Road", [1600, 1200])
            .exit_on_esc(true)
            .build()
            .unwrap();

        window.set_max_fps(60);
        window.set_ups(60);

        let textures = load_all_textures(&mut window);

        Self {
            window,
            cars: VecDeque::new(),
            textures,
            show_hit_box_and_whisker: false,
        }
    }

    pub fn render(&mut self) {
        while let Some(event) = self.window.next() {

            // Handle all keyboard events
            if let Some(Button::Keyboard(key)) = event.press_args() {

                // Toggle "hit box and whisker" debug display
                if key == Key::L {
                    self.show_hit_box_and_whisker = !self.show_hit_box_and_whisker;
                }

                // Add a new car
                if let Some(desired_direction) = handle_keyboard_event(key) {
                    // Use safe_spawning function to get an actual safe direction to spawn the car
                    if let Some(actual_direction) = safe_spawning(&self.cars, desired_direction) {
                        let texture_id = rand::thread_rng().gen_range(0..self.textures.len());
                        let id = self.cars.len();

                        // Get initial coordinates based on the actual safe direction
                        let (_init_x, _init_y) = match actual_direction {
                            Direction::East => (0, WINDOW_HEIGHT / 2 + 100),
                            Direction::North => (WINDOW_WIDTH / 2 + 100, WINDOW_HEIGHT),
                            Direction::West => (WINDOW_WIDTH, WINDOW_HEIGHT / 2 - 100),
                            Direction::South => (WINDOW_WIDTH / 2 - 100, 0),
                            Direction::NorthRight => (WINDOW_WIDTH / 2 + 150, WINDOW_HEIGHT),
                            Direction::SouthRight => (WINDOW_WIDTH / 2 - 150, 0),
                            Direction::EastRight => (0, WINDOW_HEIGHT / 2 + 150),
                            Direction::WestRight => (WINDOW_WIDTH, WINDOW_HEIGHT / 2 - 150),
                            Direction::NorthLeft => (WINDOW_WIDTH / 2 + 50, WINDOW_HEIGHT),
                            Direction::SouthLeft => (WINDOW_WIDTH / 2 - 50, 0),
                            Direction::EastLeft => (0, WINDOW_HEIGHT / 2 + 50),
                            Direction::WestLeft => (WINDOW_WIDTH, WINDOW_HEIGHT / 2 - 50),
                        };

                        let car = Car::new(id ,_init_x, _init_y, actual_direction, texture_id);
                        self.cars.push_back(car);
                    }
                }
            }


            // Draw on the canvas
            self.window.draw_2d(&event, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);

                //draw intersection zone 600x600
                draw_intersection(c, g);

                //draw direction leading lines
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
                    // Draw hit box and whisker if enabled
                    if self.show_hit_box_and_whisker {
                        draw_whisker(car, c, g);
                        // draw_hit_box(car, c, g); // This is fake skin of hit box due to library rendering.
                        // Real hit box is here
                        draw_rectangle_edges(
                            (car.hit_box.x as f32, car.hit_box.y as f32, car.hit_box.width as f32, car.hit_box.height as f32),
                            c,
                            g
                        );
                    }
                }
            });


            // Create Vec to store updated index
            let mut indices_to_update: Vec<usize> = Vec::new();
            for (idx, _car) in self.cars.iter().enumerate() {
                indices_to_update.push(idx);
            }

            // Update cars based on index and stop_frames
            for idx in indices_to_update {
                let car = &mut self.cars[idx];
                if car.stop_frames > 0 {
                    car.stop_frames -= 1;
                } else {
                    update_car_position(&mut self.cars, idx);
                }
            }

            // Update and draw hit boxes and whiskers
            for car in &mut self.cars {
                    update_whisker(car);
                    update_hit_box(car);
            }
        }
    }
}