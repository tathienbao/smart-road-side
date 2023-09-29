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




