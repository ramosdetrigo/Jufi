use macroquad::{color::Color, shapes::draw_circle_lines};

use crate::{algebra::Vec2, physics::shapes::AABB};

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

impl Circle {
    /// Gera um círculo com centro e raio definidos.
    pub fn new(center: Vec2, radius: f64) -> Circle {
        Circle { center, radius }
    }

    /// Gera um bounding circle que contém todos os pontos de um vetor.
    /// Faz isso gerando um círculo no centro de uma AABB com raio (centro -> quina)
    pub fn enclosing(points: &Vec<Vec2>) -> Circle {
        let aabb = AABB::enclosing(points);
        let center = (aabb.max + aabb.min) / 2.0;
        let radius = center.distance_to(aabb.min);
        Circle { center, radius }
    }

    #[inline]
    /// Desenha o frame da AABB na tela
    pub fn draw(&self, thickness: f32, color: Color) {
        draw_circle_lines(
            self.center.x as f32,
            self.center.y as f32,
            self.radius as f32,
            thickness,
            color,
        );
    }

    #[inline]
    /// Checa se se um ponto está dentro do círculo.
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.distance_to_squared(self.center) < self.radius * self.radius
    }

    #[inline]
    /// Checa se se um círculo está sobreposto ao outro
    pub fn overlaps_circle(&self, other: Circle) -> bool {
        let r1r2 = self.radius - other.radius;
        self.center.distance_to_squared(other.center) < r1r2 * r1r2
    }

    #[inline(always)]
    /// Checa se um círculo está sobreposto a uma AABB
    pub fn overlaps_aabb(self, other: AABB) -> bool {
        other.overlaps_circle(self)
    }
}
