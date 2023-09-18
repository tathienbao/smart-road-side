use smart_road::intersection_renderer::draw_north_right;
pub use sdl2::render::Canvas;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Smart Road", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Draw the north right intersection.
    draw_north_right(&mut canvas);

    // Assuming you also want to present the canvas and keep the window open for some time:
    canvas.present();
    std::thread::sleep(std::time::Duration::from_secs(5));
}
