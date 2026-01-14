use std::f64::INFINITY;

use macroquad::{color::Color, shapes::draw_rectangle_lines};

use crate::{
    algebra::Vec2,
    physics::shapes::{Circle, Collider, Line, OBB},
};

#[derive(Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    #[must_use]
    /// Cria uma AABB com dois pontos como limites. Esta função independe da ordem
    /// dos pontos e constrói uma AABB com min = (min_x,min_y) e max = (max_x,max_y)
    pub fn new(p1: Vec2, p2: Vec2) -> AABB {
        let (min_x, min_y) = (p1.x.min(p2.x), p1.y.min(p2.y));
        let (max_x, max_y) = (p1.x.max(p2.x), p1.y.max(p2.y));
        AABB {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        }
    }

    #[must_use]
    /// Retorna uma AABB que contém todos os pontos de um vetor. Pânico se points.len() == 0
    pub fn enclosing(points: &Vec<Vec2>) -> AABB {
        assert!(points.len() > 0, "Número de pontos deve ser maior que 0!");
        let (mut min_x, mut min_y) = (INFINITY, INFINITY);
        let (mut max_x, mut max_y) = (-INFINITY, -INFINITY);

        for p in points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        AABB {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        }
    }

    #[inline]
    #[must_use]
    /// Retorna a largura da AABB
    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    #[inline]
    #[must_use]
    /// Retorna a altura da AABB
    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    #[inline]
    /// Desenha o frame da AABB na tela
    pub fn draw(&self, thickness: f32, color: Color) {
        draw_rectangle_lines(
            self.min.x as f32,
            self.min.y as f32,
            self.width() as f32,
            self.height() as f32,
            thickness,
            color,
        );
    }

    #[inline]
    #[must_use]
    /// Checa se um ponto está dentro da bounding box
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x > self.min.x && point.x < self.max.x && point.y > self.min.y && point.y < self.max.y
    }
}

impl Collider for AABB {
    fn grow(&mut self, width: f64, height: f64) {
        let half_w = width / 2.0;
        let half_h = height / 2.0;

        self.min.x -= half_w;
        self.min.y -= half_h;

        self.max.x += half_w;
        self.max.y += half_h;
    }

    fn rotate(&mut self, _theta: f64) {}

    fn set_center(&mut self, pos: Vec2) {
        let offset = pos - self.center();
        self.max += offset;
        self.min += offset;
    }

    fn size(&self) -> Vec2 {
        self.max - self.min
    }

    fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.0
    }

    fn edges(&self) -> Vec<Line> {
        let (e1, e2) = (self.min, self.min + Vec2::X * self.width());
        let (e3, e4) = (self.max, self.max - Vec2::X * self.width());
        vec![
            Line::new(e1, e2),
            Line::new(e2, e3),
            Line::new(e3, e4),
            Line::new(e4, e1),
        ]
    }

    fn draw(&self, thickness: f32, color: Color) {
        self.draw(thickness, color);
    }

    fn project(&self, axis: Vec2) -> (f64, f64) {
        let extents = (self.max - self.min) / 2.0;
        // Projeção do centro da caixa sobre o eixo
        let center_p = self.center().dot(axis);
        // Projeção da metade da caixa sobre o eixo
        let extents_p = extents.x * axis.dot(Vec2::X).abs() + extents.y * axis.dot(Vec2::Y).abs();
        // min, max
        (center_p - extents_p, center_p + extents_p)
    }

    fn sat_axes(&self, _other: &dyn Collider) -> Vec<Vec2> {
        vec![Vec2::X, Vec2::Y]
    }
}
