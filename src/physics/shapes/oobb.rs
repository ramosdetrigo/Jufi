use rayon::prelude::*;
use std::f64::{INFINITY, consts::PI};

use macroquad::{color::Color, shapes::draw_line};

use crate::{
    algebra::Vec2,
    physics::shapes::{AABB, Circle, Collider},
};

#[derive(Clone, Copy, PartialEq)]
pub struct OOBB {
    pub center: Vec2,
    pub extents: Vec2,
    pub u: Vec2,
    pub v: Vec2,
}

impl OOBB {
    #[inline]
    #[must_use]
    /// Construtor genérico de OOBB.
    pub fn new(center: Vec2, extents: Vec2, u: Vec2, v: Vec2) -> OOBB {
        OOBB {
            center,
            extents,
            u: u.normalized(),
            v: v.normalized(),
        }
    }

    /// Cria uma OOBB que engloba todos os pontos de um vetor.
    /// Usa um algoritmo "força bruta" para gerar uma OOBB ótima.
    /// Pânico se points.len() == 0
    pub fn enclosing(points: &Vec<Vec2>) -> OOBB {
        assert!(points.len() > 0, "Número de pontos deve ser maior que 0!");
        // Testa os 180 os ângulos entre -90 e 89 para ver qual a melhor bounding box (força bruta)
        (-90..90)
            .par_bridge() // Faz as computações em paralelo usando a biblioteca Rayon
            .map(|t| OOBB::from_angle(points, (t as f64).to_radians()))
            .min_by(|a, b| a.area().total_cmp(&b.area()))
            .unwrap()
    }

    /// Retorna a área da OOBB
    pub fn area(&self) -> f64 {
        (self.extents.x * 2.0) * (self.extents.y * 2.0)
    }

    /// Função privada pra criar uma OOBB que engloba pontos com eixo U
    /// definido por um certo ângulo
    fn from_angle(points: &Vec<Vec2>, theta: f64) -> OOBB {
        // Cria um vetor U baseado em um ângulo específico
        let u = Vec2::from_angle(theta);
        let v = Vec2::new(-u.y, u.x);

        // Obtém os extents de acordo com a projeção dos pontos nos eixos
        let (min_u, max_u) = minmax_projection(points, u);
        let (min_v, max_v) = minmax_projection(points, v);
        let extents = Vec2::new(max_u - min_u, max_v - min_v) / 2.0;

        // Calcula o centro da OOBB
        let center = ((min_u + max_u) * u + (min_v + max_v) * v) / 2.0;
        OOBB::new(center, extents, u, v)
    }

    /// Retorna as 4 pontas da OOBB
    fn corners(&self) -> (Vec2, Vec2, Vec2, Vec2) {
        (
            self.center - (self.u * self.extents.x) - (self.v * self.extents.y),
            self.center + (self.u * self.extents.x) - (self.v * self.extents.y),
            self.center + (self.u * self.extents.x) + (self.v * self.extents.y),
            self.center - (self.u * self.extents.x) + (self.v * self.extents.y),
        )
    }

    /// Checa se a OOBB contém um ponto
    pub fn contains_point(&self, point: Vec2) -> bool {
        // Projeta o ponto pro espaço local da OOBB
        let p_translated = point - self.center;
        let u_proj = p_translated.dot(self.u);
        let v_proj = p_translated.dot(self.v);

        // Faz o check padrão como em uma AABB
        (-self.extents.x < u_proj && u_proj < self.extents.x)
            && (-self.extents.y < v_proj && v_proj < self.extents.y)
    }

    pub fn draw(&self, thickness: f32, color: Color) {
        let (v1, v2, v3, v4) = self.corners();

        draw_line(
            v1.x as f32,
            v1.y as f32,
            v2.x as f32,
            v2.y as f32,
            thickness,
            color,
        );
        draw_line(
            v2.x as f32,
            v2.y as f32,
            v3.x as f32,
            v3.y as f32,
            thickness,
            color,
        );
        draw_line(
            v3.x as f32,
            v3.y as f32,
            v4.x as f32,
            v4.y as f32,
            thickness,
            color,
        );
        draw_line(
            v4.x as f32,
            v4.y as f32,
            v1.x as f32,
            v1.y as f32,
            thickness,
            color,
        );
    }
}

/// Função privada para pegar os valores mínimo e máximo das projeções
/// de um vetor de pontos em um eixo.
fn minmax_projection(points: &Vec<Vec2>, axis: Vec2) -> (f64, f64) {
    let (mut min, mut max) = (INFINITY, -INFINITY);
    for p in points {
        let proj = p.dot(axis);
        min = min.min(proj);
        max = max.max(proj);
    }
    (min, max)
}

impl Collider for OOBB {
    fn center(&self) -> Vec2 {
        self.center
    }

    fn draw(&self, thickness: f32, color: Color) {
        self.draw(thickness, color);
    }

    fn project(&self, axis: Vec2) -> (f64, f64) {
        // Projeção do centro da caixa sobre o eixo
        let center_p = self.center().dot(axis);
        // Projeção da metade da caixa sobre o eixo
        let extents_p =
            self.extents.x * axis.dot(self.u).abs() + self.extents.y * axis.dot(self.v).abs();
        // min, max
        (center_p - extents_p, center_p + extents_p)
    }

    fn axes(&self) -> Vec<Vec2> {
        vec![self.u, self.v]
    }
}
