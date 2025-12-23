mod algebra; // Vector and matrix classes, etc. Algebra.
use algebra::Vec2;
mod physics;
mod utils; // Wrappers to more easily draw stuff on screen // Physics stuff: Shapes, collision detection, etc.

use crate::{
    physics::{
        generators::point_cloud,
        shapes::{AABB, Circle, OOBB},
    },
    utils::randf_range,
};
use macroquad::{color, prelude::*, rand::srand};

#[macroquad::main("Hello, World!")]
async fn main() {
    // Seta uma seed aleatória baseada no horário do sistema
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    srand(current_time);

    let mut points = point_cloud(randf_range(10, 50), 200.0, 150.0, 600.0, 450.0);
    let mut aabb = AABB::enclosing(&points);
    let mut circle = Circle::enclosing(&points);
    let mut oobb = OOBB::enclosing(&points);
    let mut mouse_point;

    loop {
        // Setup do frame atual
        clear_background(color::BLACK);
        let (mx, my) = mouse_position();
        let mouse_pos = Vec2::new(mx as f64, my as f64);
        let _delta = get_frame_time();

        mouse_point = mouse_pos;

        if is_key_pressed(KeyCode::Space) {
            points = point_cloud(randf_range(10, 50), 200.0, 150.0, 600.0, 450.0);
            aabb = AABB::enclosing(&points);
            circle = Circle::enclosing(&points);
            oobb = OOBB::enclosing(&points);
        }

        aabb.draw(2.0, color::WHITE);
        circle.draw(2.0, color::WHITE);
        oobb.draw(2.0, color::WHITE);

        for p in &points {
            p.draw(color::YELLOW);
        }

        if oobb.contains_point(mouse_point) {
            mouse_point.draw(color::PINK)
        } else if aabb.contains_point(mouse_point) {
            mouse_point.draw(color::GREEN)
        } else if circle.contains_point(mouse_point) {
            mouse_point.draw(color::BLUE)
        } else {
            mouse_point.draw(color::RED)
        }

        next_frame().await
    }
}
