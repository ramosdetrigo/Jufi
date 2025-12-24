use macroquad::{color::Color, shapes::draw_circle_lines};

use crate::{algebra::Vec2, physics::shapes::{AABB, OOBB, SATCollider}};

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

impl Circle {
    #[inline]
    #[must_use]
    /// Gera um círculo com centro e raio definidos.
    pub fn new(center: Vec2, radius: f64) -> Circle {
        Circle { center, radius }
    }

    #[must_use]
    /// Gera um bounding circle que contém todos os pontos de um vetor.
    /// Faz isso gerando um círculo no centro de uma AABB com raio (centro -> quina)
    pub fn enclosing(points: &Vec<Vec2>) -> Circle {
        assert!(points.len() > 0, "Número de pontos deve ser maior que 0!");
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
    #[must_use]
    /// Checa se se um ponto está dentro do círculo.
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.distance_to_squared(self.center) < self.radius * self.radius
    }

    #[inline]
    #[must_use]
    /// Checa se se um círculo está sobreposto ao outro
    pub fn overlaps_circle(&self, other: Circle) -> bool {
        let r1r2 = self.radius + other.radius;
        self.center.distance_to_squared(other.center) < r1r2 * r1r2
    }

    #[inline(always)]
    #[must_use]
    /// Checa se um círculo está sobreposto a uma AABB
    pub fn overlaps_aabb(self, other: AABB) -> bool {
        other.overlaps_circle(self)
    }

    #[inline(always)]
    #[must_use]
    /// Checa se um círculo está sobreposto a uma OOBB
    pub fn overlaps_oobb(self, other: OOBB) -> bool {
        other.overlaps_circle(self)
    }
}


impl SATCollider for Circle {
    fn u(&self) -> Vec2 {
        Vec2::X
    }


    fn v(&self) -> Vec2 {
        Vec2::Y
    }


    fn center(&self) -> Vec2 {
        self.center
    }


    fn draw(&self, thickness: f32, color: Color) {
        self.draw(thickness, color);
    }

    
    fn extents(&self) -> Vec2 {
        Vec2::new(self.radius, self.radius)
    }
}