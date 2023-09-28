use piston_window::*;
use crate::direction_renderer::{draw_east_right, draw_north_right, draw_south_right, draw_west_right};
use crate::object::car::Car;
use crate::object::direction::Direction;
use crate::render::car_renderer;
use crate::render::car_renderer::{draw_car, update_car_position, load_all_textures};
use crate::keyboard::handle_keyboard_event;

pub struct RendererManager {
    pub window: PistonWindow,
    pub cars: Vec<Car>,
    pub textures: Vec<G2dTexture>,
}

impl RendererManager {
    pub fn new() -> RendererManager {
        let window: PistonWindow = WindowSettings::new("Piston Demo", [800, 600])
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut factory = window.factory.clone();
        let textures = load_all_textures(&mut factory);

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

            // Draw the leading lines.
            draw_north_right(&mut self.canvas);
            draw_west_right(&mut self.canvas);
            draw_south_right(&mut self.canvas);
            draw_east_right(&mut self.canvas);

                for car in &mut self.cars {
                    draw_car(car, &self.textures, c, g);
                }
            });

            if let Some(Button::Keyboard(key)) = event.press_args() {
                if let Some(direction) = handle_keyboard_event(key) {
                    let mut car = Car::new(0, 0, direction);
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