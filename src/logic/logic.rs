use std::collections::VecDeque;
use crate::Car;
use piston_window::{rectangle, Context, G2d};

const WINDOW_WIDTH: f64 = 1600.0;
const WINDOW_HEIGHT: f64 = 1200.0;
const INTERSECTION_SIZE: f64 = 600.0;  // Size of intersection square

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

pub fn should_stop(cars: &VecDeque<Car>, current_car_id: usize) -> bool {
    let current_car = &cars[current_car_id];
    let current_whisker = &current_car.whisker;

    for (other_car_id, other_car) in cars.iter().enumerate() {
        // Don't compare the current car with itself
        if current_car_id == other_car_id {
            continue;
        }

        let other_whisker = &other_car.whisker;

        // Check if the whiskers are close to each other
        if (current_whisker.x - other_whisker.x).abs() < 10 && (current_whisker.y - other_whisker.y).abs() < 10 {
            // If the other car is closer to the intersection, stop
            if current_car_id > other_car_id {
                return true;
            }
        }
    }

    false
}

