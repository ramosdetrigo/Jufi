use macroquad::{color::Color, shapes::draw_circle_lines};

use crate::{
    algebra::Vec2,
    physics::shapes::{AABB, Collider, OOBB},
};

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
    /// Faz isso gerando um círculo no centro de uma AABB e usando a maior
    /// distância do centro até um ponto do vetor como raio.
    /// Pânico se points.len() == 0
    pub fn enclosing(points: &Vec<Vec2>) -> Circle {
        assert!(points.len() > 0, "Número de pontos deve ser maior que 0!");
        let aabb = AABB::enclosing(points);
        let center = (aabb.max + aabb.min) / 2.0;
        let radius = points
            .iter()
            .map(|p| p.distance_to_squared(center))
            .max_by(|a, b| a.total_cmp(b))
            .unwrap()
            .sqrt();
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
}

impl Collider for Circle {
    fn sat_axes(&self, other: &dyn Collider) -> Vec<Vec2> {
        // Placeholder: Implementação errada
        vec![(self.center - other.center()).normalized()]
    }

    fn center(&self) -> Vec2 {
        self.center
    }

    fn draw(&self, thickness: f32, color: Color) {
        self.draw(thickness, color);
    }

    fn project(&self, axis: Vec2) -> (f64, f64) {
        let c = self.center.dot(axis); // projeção do centro pro eixo
        (c - self.radius, c + self.radius)
    }
}
