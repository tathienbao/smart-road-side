pub use sdl2::gfx::primitives::DrawRenderer;
pub use sdl2::rect::Point;

const WINDOW_WIDTH: i32 = 800; // ví dụ, bạn có thể thay đổi giá trị này theo ý muốn
const WINDOW_HEIGHT: i32 = 600; // ví dụ, bạn có thể thay đổi giá trị này theo ý muốn
const CURVE_RADIUS_RIGHT: i32 = 50; // ví dụ, bạn có thể thay đổi giá trị này theo ý muốn

pub fn draw_north_right(renderer: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let start_point = Point::new(WINDOW_WIDTH / 2, 0);
    let curve_point = Point::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2 - CURVE_RADIUS_RIGHT as i32);
    let end_point = Point::new(WINDOW_WIDTH, WINDOW_HEIGHT / 2);

    renderer.draw_line(start_point, curve_point).unwrap();

    // Sử dụng hàm arc
    renderer.arc(
        WINDOW_WIDTH as i16 / 2, 
        (WINDOW_HEIGHT / 2 - CURVE_RADIUS_RIGHT) as i16, 
        CURVE_RADIUS_RIGHT as i16, 
        0, 
        90, // SDL2 uses degrees for arcs.
        sdl2::pixels::Color::RGB(255, 0, 0)
    ).unwrap();

    renderer.draw_line(curve_point, end_point).unwrap();
}
