use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::direction_renderer::{draw_east_right, draw_north_right, draw_south_right, draw_west_right};
use crate::car_renderer::{draw_car, update_car_position};
use crate::input::keyboard::handle_keyboard_event;
use crate::car::Car;
use crate::render::car_renderer::{draw_car, update_car_position};

pub struct RendererManager {
    sdl_context: sdl2::Sdl,
    pub canvas: Canvas<Window>,
}

impl RendererManager {
    pub fn new(sdl_context: sdl2::Sdl, canvas: Canvas<Window>) -> Self {
        RendererManager {
            sdl_context,
            canvas,
        }
    }

    pub fn render(&mut self, event_pump: &mut EventPump) {
        loop {
            // Clear the canvas
            self.canvas.clear();

            // Draw the leading lines.
            draw_north_right(&mut self.canvas);
            draw_west_right(&mut self.canvas);
            draw_south_right(&mut self.canvas);
            draw_east_right(&mut self.canvas);

            if let Some(event) = event_pump.poll_event() {
                if let Some(direction) = handle_keyboard_event(&event) {
                    draw_car(&direction, &mut self.canvas);
                }
            }

            // Update all the cars' positions
            unsafe {
                for car in &mut crate::car::CARS {
                    update_car_position(car);
                    draw_car(car, &mut self.canvas);
                }
            }

            // Present the canvas
            self.canvas.present();
        }
    }
}
