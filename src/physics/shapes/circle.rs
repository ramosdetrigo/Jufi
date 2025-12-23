use crate::algebra::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

impl Circle {
    /// Gera um círculo com centro e raio definidos.
    pub fn new(center: Vec2, radius: f64) -> Circle {
        Circle { center, radius }
    }

    /// Gera um bounding circle que contém todos os pontos de um vetor.
    /// Faz isso calculando o ponto médio do vetor e usando a maior distância entre
    /// o centro e um ponto do vetor como raio.
    pub fn enclosing(points: &Vec<Vec2>) -> Circle {
        // Pega o ponto médio de todos os pontos do vetor
        let center = points.into_iter().copied().sum::<Vec2>() / points.len() as f64;
        // Usa a maior distância entre o centro e um ponto do vetor como raio
        let radius = points
            .iter()
            .map(|p| p.distance_to(center))
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();
        Circle { center, radius }
    }

    /// Retorna true se um ponto está dentro do círculo.
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.distance_to_squared(self.center) < self.radius * self.radius
    }

    /// Retorna true se um círculo está sobreposto ao outro
    pub fn overlaps_circle(&self, other: Circle) -> bool {
        let min_distance = self.radius - other.radius;
        self.center.distance_to_squared(other.center) < min_distance * min_distance
    }
}
