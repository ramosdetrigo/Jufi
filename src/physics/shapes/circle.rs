use macroquad::{color::Color, shapes::draw_circle_lines};

use crate::{
    algebra::Vec2,
    physics::shapes::{AABB, Collider, Line, OBB},
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

    /// Retorna o ponto mais próximo do círculo em uma linha
    pub fn closest_point_on_line(&self, line: Line) -> Vec2 {
        let line_dr = line.p2 - line.p1;
        let ac = self.center - line.p1;
        let t = (ac.dot(line_dr) / line_dr.length_squared()).clamp(0.0, 1.0);
        line.p1 + line_dr * t
    }

    #[inline]
    /// (private) Retorna o eixo entre o centro do círculo e um ponto
    /// (Vetor vazio se os pontos forem iguais. Evita vetor degenerado.)
    fn sanitized_axis(&self, point: Vec2) -> Vec<Vec2> {
        if self.center.is_same(point) {
            vec![]
        } else {
            vec![(self.center - point).normalized()]
        }
    }
}

impl Collider for Circle {
    fn grow(&mut self, width: f64, height: f64) {
        if width.abs() > height.abs() {
            self.radius += width
        }
        self.radius += height
    }

    fn rotate(&mut self, _theta: f64) {}

    fn set_center(&mut self, pos: Vec2) {
        self.center = pos
    }

    fn size(&self) -> Vec2 {
        Vec2::new(self.radius, self.radius)
    }

    fn sat_axes(&self, other: &dyn Collider) -> Vec<Vec2> {
        let edges = other.edges();
        // Caso 1: Círculo -> Eixo entre os dois centros
        if edges.is_empty() {
            self.sanitized_axis(other.center())
        // Caso 2: Polígono -> Eixo entre o centro do círculo e o ponto mais próximo do polígono.
        } else {
            let closest_point = edges
                .into_iter()
                .map(|edge| self.closest_point_on_line(edge))
                .min_by(|p1, p2| {
                    let p1_dist = p1.distance_to_squared(self.center);
                    let p2_dist = p2.distance_to_squared(self.center);
                    p1_dist.total_cmp(&p2_dist)
                })
                .unwrap();
            self.sanitized_axis(closest_point)
        }
    }

    fn edges(&self) -> Vec<super::Line> {
        vec![]
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
