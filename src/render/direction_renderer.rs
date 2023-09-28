use piston_window::{Context, line, DrawState, CircleArc, G2d};

const WINDOW_WIDTH: f64 = 1600.0;
const WINDOW_HEIGHT: f64 = 1200.0;

const CURVE_RADIUS_RIGHT: f64 = 50.0;
const CURVE_RADIUS_LEFT: f64 = 120.0;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // Red color
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; // White color

pub fn draw_north_right(c: Context, g: &mut G2d) {
    // Draw the straight line from the bottom to the curve's starting point.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT, WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT / 2.0 + CURVE_RADIUS_RIGHT + 150.0], c.transform, g);

    // Draw the curve (arc) using CircleArc
    let arc = CircleArc::new(WHITE, 1.0, std::f64::consts::PI, std::f64::consts::PI * 3.0 / 2.0);
    arc.draw([WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT / 2.0 + 150.0, CURVE_RADIUS_RIGHT * 2.0, CURVE_RADIUS_RIGHT * 2.0], &DrawState::default(), c.transform, g);

    // Draw the straight line from the curve's end point to the screen's right edge.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 + CURVE_RADIUS_RIGHT + 150.0, WINDOW_HEIGHT / 2.0 + 150.0, WINDOW_WIDTH, WINDOW_HEIGHT / 2.0 + 150.0], c.transform, g);
}

pub fn draw_east_right(c: Context, g: &mut G2d) {
    // Draw the straight line from the bottom to the curve's starting point.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 - 150.0, WINDOW_HEIGHT, WINDOW_WIDTH / 2.0 - 150.0, WINDOW_HEIGHT / 2.0 + CURVE_RADIUS_RIGHT + 150.0], c.transform, g);

    // Draw the curve (arc) using CircleArc
    let arc = CircleArc::new(WHITE, 1.0, std::f64::consts::PI * 3.0 / 2.0, 0.0);
    arc.draw([WINDOW_WIDTH / 2.0 - 150.0 - CURVE_RADIUS_RIGHT * 2.0, WINDOW_HEIGHT / 2.0 + 150.0, CURVE_RADIUS_RIGHT * 2.0, CURVE_RADIUS_RIGHT * 2.0], &DrawState::default(), c.transform, g);

    // Draw the straight line from the curve's end point to the screen's left edge.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 - CURVE_RADIUS_RIGHT - 150.0, WINDOW_HEIGHT / 2.0 + 150.0, 0.0, WINDOW_HEIGHT / 2.0 + 150.0], c.transform, g);
}

pub fn draw_west_right(c: Context, g: &mut G2d) {
    // Draw the straight line from the top to the curve's starting point.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 + 150.0, 0.0, WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT / 2.0 - CURVE_RADIUS_RIGHT - 150.0], c.transform, g);

    // Draw the curve (arc) using CircleArc
    let arc = CircleArc::new(WHITE, 1.0, std::f64::consts::PI / 2.0, std::f64::consts::PI);
    arc.draw([WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT / 2.0 - 150.0 - CURVE_RADIUS_RIGHT * 2.0, CURVE_RADIUS_RIGHT * 2.0, CURVE_RADIUS_RIGHT * 2.0], &DrawState::default(), c.transform, g);

    // Draw the straight line from the curve's end point to the screen's right edge.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 + CURVE_RADIUS_RIGHT + 150.0, WINDOW_HEIGHT / 2.0 - 150.0, WINDOW_WIDTH, WINDOW_HEIGHT / 2.0 - 150.0], c.transform, g);
}

pub fn draw_south_right(c: Context, g: &mut G2d) {
    // Draw the straight line from the top to the curve's starting point.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 - 150.0, 0.0, WINDOW_WIDTH / 2.0 - 150.0, WINDOW_HEIGHT / 2.0 - CURVE_RADIUS_RIGHT - 150.0], c.transform, g);

    // Draw the curve (arc) using CircleArc
    let arc = CircleArc::new(WHITE, 1.0, 0.0, std::f64::consts::PI / 2.0);
    arc.draw([WINDOW_WIDTH / 2.0 - 150.0 - CURVE_RADIUS_RIGHT * 2.0, WINDOW_HEIGHT / 2.0 - 150.0 - CURVE_RADIUS_RIGHT * 2.0, CURVE_RADIUS_RIGHT * 2.0, CURVE_RADIUS_RIGHT * 2.0], &DrawState::default(), c.transform, g);

    // Draw the straight line from the curve's end point to the screen's left edge.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 - CURVE_RADIUS_RIGHT - 150.0, WINDOW_HEIGHT / 2.0 - 150.0, 0.0, WINDOW_HEIGHT / 2.0 - 150.0], c.transform, g);
}

pub fn draw_north(c: Context, g: &mut G2d) {
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 + 90.0, WINDOW_HEIGHT, WINDOW_WIDTH / 2.0 + 90.0, 0.0], c.transform, g);
}

pub fn draw_south(c: Context, g: &mut G2d) {
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 - 90.0, 0.0, WINDOW_WIDTH / 2.0 - 90.0, WINDOW_HEIGHT], c.transform, g);
}

pub fn draw_east(c: Context, g: &mut G2d) {
    line(WHITE, 1.0, [0.0, WINDOW_HEIGHT / 2.0 - 90.0, WINDOW_WIDTH, WINDOW_HEIGHT / 2.0 - 90.0], c.transform, g);
}

pub fn draw_west(c: Context, g: &mut G2d) {
    line(WHITE, 1.0, [WINDOW_WIDTH, WINDOW_HEIGHT / 2.0 + 90.0, 0.0, WINDOW_HEIGHT / 2.0 + 90.0], c.transform, g);
}

pub fn draw_north_left(c: Context, g: &mut G2d) {
    // Draw the straight line from the bottom to the curve's starting point.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 + 30.0, WINDOW_HEIGHT, WINDOW_WIDTH / 2.0 + 30.0, WINDOW_HEIGHT / 2.0 + 90.0], c.transform, g);

    // Draw the curve (arc) using CircleArc
    let arc = CircleArc::new(WHITE, 1.0, std::f64::consts::PI * 3.0 / 2.0, 0.0);
    arc.draw([WINDOW_WIDTH / 2.0 - 90.0 - CURVE_RADIUS_LEFT, WINDOW_HEIGHT / 2.0 + 90.0 - CURVE_RADIUS_LEFT, CURVE_RADIUS_LEFT * 2.0, CURVE_RADIUS_LEFT * 2.0], &DrawState::default(), c.transform, g);

    // Draw the straight line from the curve's end point to the screen's right edge.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 - 90.0, WINDOW_HEIGHT / 2.0 - 30.0, 0.0, WINDOW_HEIGHT / 2.0 - 30.0], c.transform, g);
}

pub fn draw_south_left(c: Context, g: &mut G2d) {
    // Draw the straight line from the top to the curve's starting point.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 - 30.0, 0.0, WINDOW_WIDTH / 2.0 - 30.0, WINDOW_HEIGHT / 2.0 - 90.0], c.transform, g);

    // Draw the curve (arc) using CircleArc
    let arc = CircleArc::new(WHITE, 1.0, std::f64::consts::PI/2.0, std::f64::consts::PI);
    arc.draw([WINDOW_WIDTH / 2.0 + 90.0 - CURVE_RADIUS_LEFT, WINDOW_HEIGHT / 2.0 - 90.0 - CURVE_RADIUS_LEFT, CURVE_RADIUS_LEFT * 2.0, CURVE_RADIUS_LEFT * 2.0], &DrawState::default(), c.transform, g);

    // Draw the straight line from the curve's end point to the screen's right edge.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 + 90.0, WINDOW_HEIGHT / 2.0 + 30.0, WINDOW_WIDTH, WINDOW_HEIGHT / 2.0 + 30.0], c.transform, g);
}

pub fn draw_west_left(c: Context, g: &mut G2d) {
    // Draw the straight line from the bottom to the curve's starting point.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 - 30.0, WINDOW_HEIGHT, WINDOW_WIDTH / 2.0 - 30.0, WINDOW_HEIGHT / 2.0 + 90.0], c.transform, g);

    // Draw the curve (arc) using CircleArc
    let arc = CircleArc::new(WHITE, 1.0, std::f64::consts::PI, std::f64::consts::PI * 3.0 / 2.0);
    arc.draw([WINDOW_WIDTH / 2.0 + 90.0 - CURVE_RADIUS_LEFT, WINDOW_HEIGHT / 2.0 + 90.0 - CURVE_RADIUS_LEFT, CURVE_RADIUS_LEFT * 2.0, CURVE_RADIUS_LEFT * 2.0], &DrawState::default(), c.transform, g);

    // Draw the straight line from the curve's end point to the screen's left edge.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 + 90.0, WINDOW_HEIGHT / 2.0 - 30.0, WINDOW_WIDTH, WINDOW_HEIGHT / 2.0 - 30.0], c.transform, g);
}

pub fn draw_east_left(c: Context, g: &mut G2d) {
    // Draw the straight line from the top to the curve's starting point.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 + 30.0, 0.0, WINDOW_WIDTH / 2.0 + 30.0, WINDOW_HEIGHT / 2.0 - 90.0], c.transform, g);

    // Draw the curve (arc) using CircleArc
    let arc = CircleArc::new(WHITE, 1.0, 0.0, std::f64::consts::PI / 2.0);
    arc.draw([WINDOW_WIDTH / 2.0 - 90.0 - CURVE_RADIUS_LEFT, WINDOW_HEIGHT / 2.0 - 90.0 - CURVE_RADIUS_LEFT, CURVE_RADIUS_LEFT * 2.0, CURVE_RADIUS_LEFT * 2.0], &DrawState::default(), c.transform, g);

    // Draw the straight line from the curve's end point to the screen's right edge.
    line(WHITE, 1.0, [WINDOW_WIDTH / 2.0 - 90.0, WINDOW_HEIGHT / 2.0 + 30.0, 0.0, WINDOW_HEIGHT / 2.0 + 30.0], c.transform, g);
}