use macroquad::{color::Color, shapes::draw_rectangle_lines};

use crate::algebra::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    /// Cria uma AABB com dois pontos como limites. Esta função independe da ordem
    /// dos pontos e constrói uma AABB com min = (min_x,min_y) e max = (max_x,max_y)
    pub fn new(p1: Vec2, p2: Vec2) -> AABB {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.y);
        let max_y = p1.y.max(p2.y);
        AABB {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        }
    }

    /// Retorna a largura da AABB
    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    /// Retorna a altura da AABB
    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

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

    /// Checa se uma bounding box está sobreposta a outra
    pub fn overlaps_aabb(&self, other: AABB) -> bool {
        !(self.max.x < other.min.x
            || self.max.y < other.min.y
            || self.min.x > other.max.x
            || self.min.y > other.max.y)
    }
}
