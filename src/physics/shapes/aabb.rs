use std::f64::INFINITY;

use macroquad::{color::Color, shapes::draw_rectangle_lines};

use crate::{algebra::Vec2, physics::shapes::Circle};

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
        let (max_x, max_y) = (p1.x.max(p2.y), p1.y.max(p2.y));
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

    #[inline]
    #[must_use]
    /// Checa se uma bounding box está sobreposta à outra
    pub fn overlaps_aabb(&self, other: AABB) -> bool {
        !(self.max.x < other.min.x
            || self.max.y < other.min.y
            || self.min.x > other.max.x
            || self.min.y > other.max.y)
    }

    #[must_use]
    /// Checa se uma AABB está sobreposta a um círculo
    pub fn overlaps_circle(&self, circle: Circle) -> bool {
        let closest_x = circle.center.x.clamp(self.min.x, self.max.x);
        let closest_y = circle.center.y.clamp(self.min.y, self.max.y);

        let dx = circle.center.x - closest_x;
        let dy = circle.center.y - closest_y;

        (dx * dx + dy * dy) <= circle.radius * circle.radius
    }
}
