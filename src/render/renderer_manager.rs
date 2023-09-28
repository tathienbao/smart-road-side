use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::direction_renderer::{draw_east_right, draw_north_right, draw_south_right, draw_west_right};
use crate::keyboard::handle_keyboard_event;
use crate::object::car::Car;
use crate::render::car_renderer::{draw_car, update_car_position};

pub struct RendererManager<'a> {
    sdl_context: sdl2::Sdl,
    pub canvas: Canvas<Window>,
    pub cars: Vec<Car>,
    event_pump: EventPump,
    textures: Vec<Texture<'a>>,
}

impl<'a> RendererManager<'a>  {
    pub fn new(sdl_context: sdl2::Sdl, textures: Vec<Texture<'a>>) -> Result<RendererManager<'a>, String> {
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("SDL2 Demo", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = sdl_context.event_pump()?;

        let cars = Vec::new();
        Ok(Self {
            sdl_context,
            canvas,
            cars,
            event_pump,
            textures,
        })
    }

    pub fn render(&mut self) {
        'running: loop {
            // Clear the canvas
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            // Draw the leading lines.
            draw_north_right(&mut self.canvas);
            draw_west_right(&mut self.canvas);
            draw_south_right(&mut self.canvas);
            draw_east_right(&mut self.canvas);

            // Input Handler
            // Initialize should_quit as false at the beginning of each loop iteration
            let mut should_quit = false;

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        should_quit = true;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        should_quit = true;
                    }
                    _ => {}
                }
                if let Some(direction) = handle_keyboard_event(&event) {
                    let mut car = Car::new(0, 0, direction);
                    draw_car(&mut car, &mut self.canvas, &self.textures);
                }
            }
            // If should_quit is true, break the loop
            if should_quit {
                break 'running;
            }

            // Update all the cars' positions
            for car in &mut self.cars {
                update_car_position(car);
                draw_car(car, &mut self.canvas, &self.textures);
            }

            // Present the canvas
            self.canvas.present();
        }
    }
}
