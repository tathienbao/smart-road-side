use std::collections::VecDeque;
use crate::Car;
use piston_window::{rectangle, Context, G2d, line};

const WINDOW_WIDTH: f64 = 1600.0;
const WINDOW_HEIGHT: f64 = 1200.0;
const INTERSECTION_SIZE: f64 = 400.0;
const CURVE_RADIUS_RIGHT: f64 = 50.0;


/// INTERSECTION LOGIC
// Check if a car is in the intersection
pub fn in_intersection(car: &Car) -> bool {
    let x_center = WINDOW_WIDTH / 2.0;
    let y_center = WINDOW_HEIGHT / 2.0;

    let x_min = x_center - (INTERSECTION_SIZE / 2.0);
    let x_max = x_center + (INTERSECTION_SIZE / 2.0);
    let y_min = y_center - (INTERSECTION_SIZE / 2.0);
    let y_max = y_center + (INTERSECTION_SIZE / 2.0);

    (car.x as f64 >= x_min) && (car.x as f64 <= x_max) &&
        (car.y as f64 >= y_min) && (car.y as f64 <= y_max)
}

// Draw the intersection area for debugging purposes
pub fn draw_intersection(c: Context, g: &mut G2d) {
    let intersection_color = [0.2, 0.2, 0.2, 1.0]; // RGBA, green and semi-transparent
    let x_min = (WINDOW_WIDTH / 2.0 - INTERSECTION_SIZE / 2.0) as f64;
    let y_min = (WINDOW_HEIGHT / 2.0 - INTERSECTION_SIZE / 2.0) as f64;

    rectangle(
        intersection_color,
        [x_min, y_min, INTERSECTION_SIZE as f64, INTERSECTION_SIZE as f64],
        c.transform,
        g,
    );
}


/// HIT BOX AND WHISKER LOGIC
///
// Define the hit box borders for a car
fn line_intersects_rect(line_p1: (f32, f32), line_p2: (f32, f32), rect: (f32, f32, f32, f32)) -> bool {
    // Rectangle edges
    let (rect_x, rect_y, rect_w, rect_h) = rect;

    let edges = [
        ((rect_x, rect_y), (rect_x + rect_w, rect_y)), // Top
        ((rect_x + rect_w, rect_y), (rect_x + rect_w, rect_y + rect_h)), // Right
        ((rect_x, rect_y + rect_h), (rect_x + rect_w, rect_y + rect_h)), // Bottom
        ((rect_x, rect_y), (rect_x, rect_y + rect_h)), // Left
    ];

    // Check each edge
    for &(a, b) in &edges {
        if line_line_intersection(line_p1, line_p2, a, b) {
            return true;
        }
    }

    false
}

// Check if two lines intersect each other
fn line_line_intersection(a1: (f32, f32), a2: (f32, f32), b1: (f32, f32), b2: (f32, f32)) -> bool {
    let (x1, y1) = a1;
    let (x2, y2) = a2;
    let (x3, y3) = b1;
    let (x4, y4) = b2;

    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    // Lines are parallel or too close to parallel
    if denominator.abs() < 1e-10 {
        return false;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        return true;
    }

    false
}

// Draw to display precision of the cut rectangular
pub fn draw_rectangle_edges(rect: (f32, f32, f32, f32), c: Context, g: &mut G2d) {
    let (x, y, w, h) = rect;

    // Create a vector of the rectangle's vertices
    let vertices = [(x, y), (x + w, y), (x + w, y + h), (x, y + h)];

    let yellow = [1.0, 1.0, 0.0, 1.0];

    // Draw a line along the edges of the rectangle
    for i in 0..4 {
        let j = (i + 1) % 4;
        line(
            yellow,
            1.0, // Thickness
            [vertices[i].0 as f64, vertices[i].1 as f64, vertices[j].0 as f64, vertices[j].1 as f64],
            c.transform,
            g,
        );
    }
}

/// TURNING CAR LOGIC
///
// Update the car's position based on Bezier curve
use kurbo::{ParamCurve, Point};
use crate::direction::Direction;
pub fn perform_turn(car: &mut Car) {
    if car.turn_progress >= 1.0 {
        return; // Done turning
    }

    // Determine the rate of turn based on direction
    let turn_rate = match car.direction {
        Direction::NorthRight | Direction::SouthRight | Direction::EastRight | Direction::WestRight => 1.0 / 60.0, // 1 second to complete the turn
        Direction::NorthLeft | Direction::SouthLeft | Direction::EastLeft | Direction::WestLeft => 1.0 / 120.0, // 2 seconds to complete the turn
        _ => 1.0 / 60.0, // Default to 1 second if direction does not involve turning
    };

    // Increment turn progress based on the calculated rate
    car.turn_progress += turn_rate;
    if car.turn_progress > 1.0 {
        car.turn_progress = 1.0;
    }

    // Rest of the code remains the same
    let t = car.turn_progress;

    // Declare control points for cubic Bézier curve
    let (p0, p1, p2, p3) = match car.direction {
        Direction::NorthRight => (
            Point::new(WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT / 2.0 + CURVE_RADIUS_RIGHT + 150.0),
            Point::new(WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT / 2.0 + CURVE_RADIUS_RIGHT + 150.0 - 30.0),
            Point::new(WINDOW_WIDTH / 2.0 + 150.0 + 20.0, WINDOW_HEIGHT / 2.0 + 150.0),
            Point::new(WINDOW_WIDTH / 2.0 + 151.0 + CURVE_RADIUS_RIGHT, WINDOW_HEIGHT / 2.0 + 150.0)
        ),
        Direction::SouthRight => (
            Point::new(WINDOW_WIDTH / 2.0 - 150.0, WINDOW_HEIGHT / 2.0 - 150.0 - CURVE_RADIUS_RIGHT ),
            Point::new(WINDOW_WIDTH / 2.0 - 150.0, WINDOW_HEIGHT / 2.0 - 150.0 - CURVE_RADIUS_RIGHT + 30.0),
            Point::new(WINDOW_WIDTH / 2.0 - 150.0 - 20.0, WINDOW_HEIGHT / 2.0 - 150.0),
            Point::new(WINDOW_WIDTH / 2.0 - 151.0 - CURVE_RADIUS_RIGHT, WINDOW_HEIGHT / 2.0 - 150.0)
        ),
        Direction::EastRight => (
            Point::new(WINDOW_WIDTH / 2.0 - 150.0 - CURVE_RADIUS_RIGHT, WINDOW_HEIGHT / 2.0 + 150.0),
            Point::new(WINDOW_WIDTH / 2.0 - 150.0 - CURVE_RADIUS_RIGHT + 30.0, WINDOW_HEIGHT / 2.0 + 150.0),
            Point::new(WINDOW_WIDTH / 2.0 - 150.0, WINDOW_HEIGHT / 2.0 + 150.0 + 30.0),
            Point::new(WINDOW_WIDTH / 2.0 - 150.0, WINDOW_HEIGHT / 2.0 + 151.0 + CURVE_RADIUS_RIGHT)
        ),
        Direction::WestRight => (
            Point::new(WINDOW_WIDTH / 2.0 + 150.0 + CURVE_RADIUS_RIGHT, WINDOW_HEIGHT / 2.0 - 150.0),
            Point::new(WINDOW_WIDTH / 2.0 + 150.0 + CURVE_RADIUS_RIGHT - 30.0, WINDOW_HEIGHT / 2.0 - 150.0),
            Point::new(WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT / 2.0 - 150.0 - 30.0),
            Point::new(WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT / 2.0 - 151.0 - CURVE_RADIUS_RIGHT)
        ),
        Direction::NorthLeft => (
            Point::new(WINDOW_WIDTH / 2.0 + 50.0, WINDOW_HEIGHT / 2.0 + 200.0),
            Point::new(WINDOW_WIDTH / 2.0 + 50.0, WINDOW_HEIGHT / 2.0 + 200.0 - 120.0),
            Point::new(WINDOW_WIDTH / 2.0 + 50.0 - 120.0, WINDOW_HEIGHT / 2.0 - 50.0),
            Point::new(WINDOW_WIDTH / 2.0 - 201.0, WINDOW_HEIGHT / 2.0 - 50.0)
        ),
        Direction::SouthLeft => (
            Point::new(WINDOW_WIDTH / 2.0 - 50.0, WINDOW_HEIGHT / 2.0 - 200.0),
            Point::new(WINDOW_WIDTH / 2.0 - 50.0, WINDOW_HEIGHT / 2.0 - 200.0 + 120.0),
            Point::new(WINDOW_WIDTH / 2.0 - 50.0 + 120.0, WINDOW_HEIGHT / 2.0 + 50.0),
            Point::new(WINDOW_WIDTH / 2.0 + 201.0, WINDOW_HEIGHT / 2.0 + 50.0)
        ),
        Direction::EastLeft => (
            Point::new(WINDOW_WIDTH / 2.0 - 200.0, WINDOW_HEIGHT / 2.0 + 50.0),
            Point::new(WINDOW_WIDTH / 2.0 - 200.0 + 120.0, WINDOW_HEIGHT / 2.0 + 50.0),
            Point::new(WINDOW_WIDTH / 2.0 + 50.0, WINDOW_HEIGHT / 2.0 + 50.0 - 120.0),
            Point::new(WINDOW_WIDTH / 2.0 + 50.0, WINDOW_HEIGHT / 2.0 - 201.0)
        ),
        Direction::WestLeft => (
            Point::new(WINDOW_WIDTH / 2.0 + 200.0, WINDOW_HEIGHT / 2.0 - 50.0),
            Point::new(WINDOW_WIDTH / 2.0 + 200.0 - 120.0, WINDOW_HEIGHT / 2.0 - 50.0),
            Point::new(WINDOW_WIDTH / 2.0 - 50.0, WINDOW_HEIGHT / 2.0 - 50.0 + 120.0),
            Point::new(WINDOW_WIDTH / 2.0 - 50.0, WINDOW_HEIGHT / 2.0 + 201.0)
        ),
           _ => (
                Point::new(WINDOW_WIDTH / 2.0 + 100.0, WINDOW_HEIGHT),
                Point::new(WINDOW_WIDTH / 2.0 + 100.0, WINDOW_HEIGHT / 2.0 + 100.0),
                Point::new(WINDOW_WIDTH / 2.0 + 100.0, WINDOW_HEIGHT / 2.0 + 100.0),
                Point::new(WINDOW_WIDTH / 2.0 + 100.0, WINDOW_HEIGHT / 2.0 + 100.0)
            ),
    };

    // Create a cubic Bézier curve
    let curve = kurbo::CubicBez::new(p0, p1, p2, p3);


    // get the position of the car at t
    let pos = curve.eval(t);

    car.x = pos.x as i32;
    car.y = pos.y as i32;
}


/// THIS SECTION IS FOR PRIORITY QUEUE LOGIC
/// WE HAVE 4 FUNCTIONS.
// For cars at slow speed
pub fn should_stop(cars: &VecDeque<Car>, current_car_id: usize) -> bool {
    let current_car = &cars[current_car_id];
    let current_whisker_start = (current_car.x as f32, current_car.y as f32);
    let current_whisker_end = (current_car.whisker.x as f32, current_car.whisker.y as f32);

    for (other_car_id, other_car) in cars.iter().enumerate() {
        if current_car_id == other_car_id {
            continue;
        }

        let other_hit_box = &other_car.hit_box;
        let rect = (
            other_hit_box.x as f32,
            other_hit_box.y as f32,
            other_hit_box.width as f32,
            other_hit_box.height as f32,
        );

        if line_intersects_rect(current_whisker_start, current_whisker_end, rect) {
            return true;
        }
    }

    false
}

// For cars at high speed
pub fn slow_down(car: &mut Car) {
    // Reduce the speed of the car to a safer level
}

// For cars that enter the intersection
pub fn update_intersection_status(cars: &mut VecDeque<Car>, insiders: &mut VecDeque<usize>) {
    // Check if a car has entered the intersection
    // If yes, mark it as `is_inside` and add it to `Insiders`
}

// Check conflict by directions
pub fn check_conflict_by_direction(insiders: &VecDeque<usize>, cars: &VecDeque<Car>) -> bool {
    // Check for conflicts in direction among the cars in `Insiders`
    false
}

/// Returns an available direction for spawning a new car, or None if no direction is available.
const SAFE_SPAWN_DISTANCE: i32 = 100;
pub fn safe_spawning(cars: &VecDeque<Car>, desired_direction: Direction) -> Option<Direction> {
    let mut available_directions: Vec<Direction> = vec![];

    // Determine the possible directions based on the desired direction
    match desired_direction {
        Direction::North => {
            available_directions.push(Direction::North);
            available_directions.push(Direction::NorthRight);
            available_directions.push(Direction::NorthLeft);
        },
        Direction::East => {
            available_directions.push(Direction::East);
            available_directions.push(Direction::EastRight);
            available_directions.push(Direction::EastLeft);
        },
        Direction::South => {
            available_directions.push(Direction::South);
            available_directions.push(Direction::SouthRight);
            available_directions.push(Direction::SouthLeft);
        },
        Direction::West => {
            available_directions.push(Direction::West);
            available_directions.push(Direction::WestRight);
            available_directions.push(Direction::WestLeft);
        },
        _ => return None, // Invalid or unsupported direction
    }

    let (init_x, init_y) = get_initial_coordinates(desired_direction);

    for car in cars.iter() {
        if let Some(index) = available_directions.iter().position(|dir| dir == &car.direction) {
            // Check if the car is too close to the spawn point
            let dx = (car.x - init_x as i32).abs();
            let dy = (car.y - init_y as i32).abs();
            if dx <= SAFE_SPAWN_DISTANCE && dy <= SAFE_SPAWN_DISTANCE {
                // Delete the direction from the list of available directions
                available_directions.remove(index);
            }
        }

        // No available directions
        if available_directions.is_empty() {
            return None;
        }
    }

    // Return the first available direction
    available_directions.first().cloned()
}

pub fn get_initial_coordinates(direction: Direction) -> (f64, f64) {
    match direction {
        Direction::North => (WINDOW_WIDTH / 2.0 + 100.0, WINDOW_HEIGHT),
        Direction::East => (0.0, WINDOW_HEIGHT / 2.0 + 100.0),
        Direction::South => (WINDOW_WIDTH / 2.0 - 100.0, 0.0),
        Direction::West => (WINDOW_WIDTH, WINDOW_HEIGHT / 2.0 - 100.0),
        Direction::NorthRight => (WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT),
        _ => (0.0, 0.0),
    }
}