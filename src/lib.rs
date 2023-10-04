pub mod object {
    pub mod car;
    pub mod direction;
    // and others
}

pub mod render {
    pub mod car_renderer;
    pub mod direction_renderer;
    pub mod renderer_manager;
    pub mod text;
}

pub mod input;
pub mod logic;

pub use crate::render::direction_renderer;
pub use object::car::Car;
pub use object::direction;
pub use input::keyboard;