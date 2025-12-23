use rayon::prelude::*;
use std::f64::{INFINITY, consts::PI};

use macroquad::{color::Color, shapes::draw_line};

use crate::algebra::Vec2;

pub struct OOBB {
    pub center: Vec2,
    pub extents: Vec2,
    pub u: Vec2,
    pub v: Vec2,
}

impl OOBB {
    #[inline]
    #[must_use]
    pub fn new(center: Vec2, extents: Vec2, u: Vec2, v: Vec2) -> OOBB {
        OOBB {
            center,
            extents,
            u: u.normalized(),
            v: v.normalized(),
        }
    }

    pub fn enclosing(points: &Vec<Vec2>) -> OOBB {
        assert!(points.len() > 0, "Número de pontos deve ser maior que 0!");
        (-90..90)
            .par_bridge()
            .map(|t| OOBB::from_angle(points, (t as f64).to_radians()))
            .min_by(|a, b| a.area().total_cmp(&b.area()))
            .unwrap()
    }

    pub fn area(&self) -> f64 {
        (self.extents.x * 2.0) * (self.extents.y * 2.0)
    }

    fn from_angle(points: &Vec<Vec2>, theta: f64) -> OOBB {
        let u = Vec2::from_angle(theta);
        let v = Vec2::new(-u.y, u.x);

        let (min_u, max_u) = minmax_projection(points, u);
        let (min_v, max_v) = minmax_projection(points, v);

        let extents = Vec2::new(max_u - min_u, max_v - min_v) / 2.0;
        let center = ((min_u + max_u) * u + (min_v + max_v) * v) / 2.0;

        OOBB::new(center, extents, u, v)
    }

    fn corners(&self) -> (Vec2, Vec2, Vec2, Vec2) {
        (
            self.center - (self.u * self.extents.x) - (self.v * self.extents.y),
            self.center + (self.u * self.extents.x) - (self.v * self.extents.y),
            self.center + (self.u * self.extents.x) + (self.v * self.extents.y),
            self.center - (self.u * self.extents.x) + (self.v * self.extents.y),
        )
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        let p_translated = point - self.center;
        let u_proj = p_translated.dot(self.u);
        let v_proj = p_translated.dot(self.v);

        (-self.extents.x < u_proj && u_proj < self.extents.x)
        && (-self.extents.y < v_proj && v_proj < self.extents.y)
    }

    pub fn draw(&self, thickness: f32, color: Color) {
        let (v1, v2, v3, v4) = self.corners();

        draw_line(v1.x as f32, v1.y as f32, v2.x as f32, v2.y as f32, thickness, color);
        draw_line(v2.x as f32, v2.y as f32, v3.x as f32, v3.y as f32, thickness, color);
        draw_line(v3.x as f32, v3.y as f32, v4.x as f32, v4.y as f32, thickness, color);
        draw_line(v4.x as f32, v4.y as f32, v1.x as f32, v1.y as f32, thickness, color);
    }
}

fn minmax_projection(points: &Vec<Vec2>, axis: Vec2) -> (f64, f64) {
    let (mut min, mut max) = (INFINITY, -INFINITY);
    for p in points {
        let proj = p.dot(axis);
        min = min.min(proj);
        max = max.max(proj);
    }
    (min, max)
}
