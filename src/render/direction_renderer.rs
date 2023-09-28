extern crate piston_window;
use piston_window::*;
use graphics::types::Color;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;
const CURVE_RADIUS_RIGHT: f64 = 50.0;
const RED: Color = [1.0, 0.0, 0.0, 1.0]; // RGBA

pub fn draw_north_right(c: Context, g: &mut G2d) {
    // Draw the straight line from the bottom to the curve's starting point.
    line(RED, 1.0, [WINDOW_WIDTH / 2.0, WINDOW_HEIGHT, WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0 + CURVE_RADIUS_RIGHT], c.transform, g);

    // Draw the curve (arc)
    arc(RED, 1.0, 0.0, 1.57, // 0 to pi/2 radians
        [WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, CURVE_RADIUS_RIGHT * 2.0, CURVE_RADIUS_RIGHT * 2.0],
        c.transform, g);

    // Draw the straight line from the curve's end point to the screen's right edge.
    line(RED, 1.0, [WINDOW_WIDTH / 2.0 + CURVE_RADIUS_RIGHT, WINDOW_HEIGHT / 2.0, WINDOW_WIDTH, WINDOW_HEIGHT / 2.0], c.transform, g);
}

// Repeat similar steps for draw_east_right, draw_west_right, and draw_south_right
