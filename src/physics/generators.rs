use std::f64::consts::PI;

use crate::{algebra::Vec2, utils::randf_range};

#[inline]
/// Gera um ponto aleatório em um retângulo (min_x,min_y) (max_x,max_y)
pub fn random_point(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Vec2 {
    Vec2::new(randf_range(min_x, max_x), randf_range(min_y, max_y))
}

#[inline]
/// Gera um ponto aleatório em um raio ao redor de um ponto
pub fn random_point_radial(center: Vec2, radius: f64) -> Vec2 {
    let angle = randf_range(-PI, PI);
    Vec2::new(
        angle.sin() * randf_range(0.0, radius),
        angle.cos() * randf_range(0.0, radius),
    ) + center
}

#[inline]
/// Gera uma nuvem de N pontos em um retângulo (min_x,min_y) (max_x,max_y)
pub fn point_cloud(n: usize, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        points.push(random_point(min_x, min_y, max_x, max_y));
    }
    points
}

#[inline]
/// Gera uma nuvem de N pontos em um raio ao redor de um ponto
pub fn point_cloud_radial(n: usize, center: Vec2, radius: f64) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        points.push(random_point_radial(center, radius))
    }
    points
}
