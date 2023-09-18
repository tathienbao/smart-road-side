pub use sdl2::gfx::primitives::DrawRenderer;
pub use sdl2::rect::Point;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const CURVE_RADIUS_RIGHT: i32 = 50;

pub fn draw_north_right(renderer: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    // Calculate the curve's center point.
    let curve_center = Point::new(
        WINDOW_WIDTH / 2 + CURVE_RADIUS_RIGHT,
        WINDOW_HEIGHT / 2 + CURVE_RADIUS_RIGHT,
    );

    // Define start, curve, and end points
    let start_point = Point::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT);
    let curve_start_point = Point::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2 + CURVE_RADIUS_RIGHT);
    let curve_end_point = Point::new(WINDOW_WIDTH / 2 + CURVE_RADIUS_RIGHT, WINDOW_HEIGHT / 2);
    let end_point = Point::new(WINDOW_WIDTH, WINDOW_HEIGHT / 2);

    // Draw the straight line from the bottom to the curve's starting point.
    renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    renderer.draw_line(start_point, curve_start_point).unwrap();

    // Draw the curve (arc)
    renderer.arc(
        curve_center.x() as i16,
        curve_center.y() as i16,
        CURVE_RADIUS_RIGHT as i16,
        180, 
        270, 
        sdl2::pixels::Color::RGB(255, 0, 0),
    ).unwrap();

    // Draw the straight line from the curve's end point to the screen's right edge.
    renderer.draw_line(curve_end_point, end_point).unwrap();
}

pub fn draw_east_right(renderer: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let curve_center = Point::new(
        WINDOW_WIDTH / 2 - CURVE_RADIUS_RIGHT,
        WINDOW_HEIGHT / 2 + CURVE_RADIUS_RIGHT,
    );

    let start_point = Point::new(0, WINDOW_HEIGHT / 2);
    let curve_start_point = Point::new(WINDOW_WIDTH / 2 - CURVE_RADIUS_RIGHT, WINDOW_HEIGHT / 2);
    let curve_end_point = Point::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2 + CURVE_RADIUS_RIGHT);
    let end_point = Point::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT);

    renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    renderer.draw_line(start_point, curve_start_point).unwrap();
    renderer.arc(
        curve_center.x() as i16,
        curve_center.y() as i16,
        CURVE_RADIUS_RIGHT as i16,
        270,   
        0,       
        sdl2::pixels::Color::RGB(255, 0, 0),
    ).unwrap();
    renderer.draw_line(curve_end_point, end_point).unwrap();
}

pub fn draw_west_right(renderer: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let curve_center = Point::new(
        WINDOW_WIDTH / 2 + CURVE_RADIUS_RIGHT,
        WINDOW_HEIGHT / 2 - CURVE_RADIUS_RIGHT,
    );

    let start_point = Point::new(WINDOW_WIDTH, WINDOW_HEIGHT / 2);
    let curve_start_point = Point::new(WINDOW_WIDTH / 2 + CURVE_RADIUS_RIGHT, WINDOW_HEIGHT / 2);
    let curve_end_point = Point::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2 - CURVE_RADIUS_RIGHT);
    let end_point = Point::new(WINDOW_WIDTH / 2, 0);

    renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    renderer.draw_line(start_point, curve_start_point).unwrap();
    renderer.arc(
        curve_center.x() as i16,
        curve_center.y() as i16,
        CURVE_RADIUS_RIGHT as i16,
        90,  
        180,   
        sdl2::pixels::Color::RGB(255, 0, 0),
    ).unwrap();
    renderer.draw_line(curve_end_point, end_point).unwrap();
}

pub fn draw_south_right(renderer: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let curve_center = Point::new(
        WINDOW_WIDTH / 2 - CURVE_RADIUS_RIGHT,
        WINDOW_HEIGHT / 2 - CURVE_RADIUS_RIGHT,
    );

    let start_point = Point::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT);
    let curve_start_point = Point::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2 + CURVE_RADIUS_RIGHT);
    let curve_end_point = Point::new(WINDOW_WIDTH / 2 - CURVE_RADIUS_RIGHT, WINDOW_HEIGHT / 2);
    let end_point = Point::new(0, WINDOW_HEIGHT / 2);

    renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    renderer.draw_line(start_point, curve_start_point).unwrap();
    renderer.arc(
        curve_center.x() as i16,
        curve_center.y() as i16,
        CURVE_RADIUS_RIGHT as i16,
        0,    
        90,   
        sdl2::pixels::Color::RGB(255, 0, 0),
    ).unwrap();
    renderer.draw_line(curve_end_point, end_point).unwrap();
}
