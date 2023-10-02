use std::collections::VecDeque;
use piston_window::{Context, G2d, text, Transformed};
use piston_window::types::Color;
use crate::Car;

const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

// Trong hàm vẽ hoặc một phần nào đó của logic vẽ
pub fn draw_insider_indices(c: Context, g: &mut G2d, cars: &VecDeque<Car>, insiders: &VecDeque<usize>) {
    let character_cache = // Khởi tạo hoặc truy cập vào GlyphCache tại đây
        for &insider_id in insiders.iter() {
            let car = &cars[insider_id];
            let transform = c.transform.trans(car.x as f64, car.y as f64);

            text(WHITE, 16, &insider_id.to_string(), character_cache, transform, g).unwrap();
        };
}