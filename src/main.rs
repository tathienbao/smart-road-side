use smart_road::intersection_renderer::draw_north_right;
use smart_road::intersection_renderer::draw_west_right;
use smart_road::intersection_renderer::draw_south_right;
use smart_road::intersection_renderer::draw_east_right;
pub use sdl2::render::Canvas;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Smart Road", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit { .. } => break 'running,
                _ => {},
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));  // Set the canvas color to black
        canvas.clear();  // Clear the canvas
        
        // Draw the north right intersection.
        draw_north_right(&mut canvas);
        draw_west_right(&mut canvas);
        draw_south_right(&mut canvas);
        draw_east_right(&mut canvas);

        // Assuming you also want to present the canvas and keep the window open for some time:
        canvas.present();

        // Optional: If you want to limit the frame rate
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
