use crate::algebra::Vec2;
pub mod shapes;
pub mod generators;

pub struct Intersection {
    /// O t da interseção em r1(t) (interseção do "raio reta1" com a reta2)
    pub t: f64,
    /// O u da interseção em r2(t) (interseção do "raio reta2" com a reta1)
    pub u: f64,
    /// O ponto da interseção
    pub p: Vec2,
    /// A normal da interseção
    pub normal: Vec2,
}
