use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::thread;
use std::time::Duration;
use crate::input::keyboard::handle_keyboard_event;
use crate::render::car_renderer::draw_car;

use super::direction_renderer::{
                                    draw_north_right,
                                    draw_west_right,
                                    draw_south_right,
                                    draw_east_right,
};

pub struct RendererManager {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,

}

impl RendererManager {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Smart Road", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        RendererManager {
            sdl_context,
            canvas,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));  // Set the canvas color to black
            self.canvas.clear();  // Clear the canvas

            // Draw the intersections.
            draw_north_right(&mut self.canvas);
            draw_west_right(&mut self.canvas);
            draw_south_right(&mut self.canvas);
            draw_east_right(&mut self.canvas);

            for event in event_pump.poll_iter() {
                // Car spawning
                if let Some(direction) = handle_keyboard_event(&event) {
                    draw_car(direction, &mut self.canvas);
                }

                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {},
                }
            }

            self.canvas.present();

            // Optional: If you want to limit the frame rate
            thread::sleep(Duration::from_millis(100));
        }
    }

}
