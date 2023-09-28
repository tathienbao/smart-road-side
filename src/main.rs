use smart_road::render::renderer_manager::RendererManager;
use sdl2::init;

fn main() {
    let sdl_context = init().unwrap();
    let manager = RendererManager::new(sdl_context, vec![]);
    manager.expect("COULD NOT RENDER").render();
}
