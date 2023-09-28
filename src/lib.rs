pub mod object {
    pub mod car;
    pub mod direction;
    // and others
}

pub mod render {
    pub mod car_renderer;
    pub mod direction_renderer;
    pub mod renderer_manager;
   // and others
}

pub mod input;

pub use sdl2::video::Window;
pub use crate::render::direction_renderer;
pub use object::car::Car;
pub use object::car::CARS;
pub use object::direction;
pub use input::keyboard;