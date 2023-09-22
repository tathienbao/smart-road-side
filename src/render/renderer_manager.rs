use crate::direction::Direction;
use crate::render::car_renderer::draw_car;
use crate::render::direction_renderer::{draw_east_right, draw_north_right, draw_south_right, draw_west_right};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use std::thread;
use crate::Car;
use crate::keyboard::handle_keyboard_event;

pub struct RendererManager {
    sdl_context: sdl2::Sdl,
    pub canvas: Canvas<Window>,
    car: Option<Car>,
}

impl RendererManager {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("smart-road", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        RendererManager { sdl_context, canvas, car: None, }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            // Clear the canvas with the background color
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            // Draw the intersections.
            draw_north_right(&mut self.canvas);
            draw_west_right(&mut self.canvas);
            draw_south_right(&mut self.canvas);
            draw_east_right(&mut self.canvas);

            for event in event_pump.poll_iter() {
                // Handle car spawning based on keyboard input
                if let Some(direction) = handle_keyboard_event(&event) {
                    // Here you create or update the car based on the direction
                    let (x, y) = match direction {
                        Direction::North => (400, 600),
                        Direction::South => (400, 0),
                        Direction::East => (0, 300),
                        Direction::West => (800, 300),
                        _ => (0,0) // default
                    };
                    self.car = Some(Car::new(x, y, direction));
                }

                // Check for quit event
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {},
                }
            }

            // If there's a car, draw it
            if let Some(car) = &mut self.car {
                car.update_position();
                draw_car(car, &mut self.canvas);
            }

            // Present the drawn frame to the screen
            self.canvas.present();

            // Optional: If you want to limit the frame rate
            thread::sleep(Duration::from_millis(8));
        }
    }
}
