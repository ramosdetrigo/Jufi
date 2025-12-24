use macroquad::color::Color;

use crate::physics::shapes::{AABB, Circle, OOBB};

#[derive(Clone, Copy, PartialEq)]
pub enum Collider {
    Circle(Circle),
    AABB(AABB),
    OOBB(OOBB)
}

impl Collider {
    pub fn collides_with(&self, other: Collider) -> bool {
        match (self, other) {
            // Self: Circle
            (Collider::Circle(a), Collider::Circle(b)) => a.overlaps_circle(b),
            (Collider::Circle(a), Collider::AABB(b))   => a.overlaps_aabb(b),
            (Collider::Circle(a), Collider::OOBB(b))   => a.overlaps_oobb(b),

            // Self: AABB
            (Collider::AABB(a),   Collider::Circle(b)) => a.overlaps_circle(b),
            (Collider::AABB(a),   Collider::AABB(b))   => a.overlaps_aabb(b),
            (Collider::AABB(a),   Collider::OOBB(b))   => a.overlaps_oobb(b),

            // Self: OOBB
            (Collider::OOBB(a),   Collider::Circle(b)) => a.overlaps_circle(b),
            (Collider::OOBB(a),   Collider::OOBB(b)) => a.overlaps_oobb(b),
            (Collider::OOBB(a),   Collider::AABB(b)) => a.overlaps_aabb(b),
        }
    }

    pub fn draw(&self, thickness: f32, color: Color) {
        match self {
            Collider::Circle(s) => s.draw(thickness, color),
            Collider::AABB(s) => s.draw(thickness, color),
            Collider::OOBB(s) => s.draw(thickness, color),
        }
    }
}