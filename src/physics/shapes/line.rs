use crate::{algebra::Vec2, physics::Intersection};
use macroquad::{color::Color, shapes::draw_line};

/// Struct para uma linha em um espaço 2D
#[derive(Clone, Copy, PartialEq)]
pub struct Line {
    pub p1: Vec2,
    pub p2: Vec2,
}

// Métodos
impl Line {
    #[inline]
    #[must_use]
    /// Construtor da linha
    pub fn new(p1: Vec2, p2: Vec2) -> Line {
        return Line { p1, p2 };
    }

    /// Desenha a linha na tela
    pub fn draw(&self, thickness: f32, color: Color) {
        draw_line(
            self.p1.x as f32,
            self.p1.y as f32,
            self.p2.x as f32,
            self.p2.y as f32,
            thickness,
            color,
        );
    }

    #[must_use]
    /// Retorna um vetor normal à linha p1 - p2 (-90°)
    pub fn normal(&self) -> Vec2 {
        let v = self.p2 - self.p1; // vetor de p1 pra p2
        let n = Vec2::new(v.y, -v.x);
        return n.normalized();
    }

    #[inline]
    #[must_use]
    /// Retorna o tamanho da reta ao quadrado (mais rápido que length() * length())
    pub fn length_squared(&self) -> f64 {
        return (self.p1 - self.p2).length_squared();
    }

    #[inline]
    #[must_use]
    /// Retorna o tamanho da reta
    pub fn length(&self) -> f64 {
        return (self.p1 - self.p2).length();
    }

    #[inline]
    #[must_use]
    /// Retorna se a reta é degenerada ou não testando se
    /// a distância entre seus dois pontos é igual a 0 (threshold 1e-6)
    pub fn is_degenerate(&self) -> bool {
        return self.length_squared() <= 1e-12;
    }

    #[must_use]
    /// Checa se uma reta é paralela com a outra através do produto vetorial.
    /// Se o produto vetorial der igual a 0, elas são paralalas. (Threshold 1e-8)
    pub fn is_parallel_with(&self, other: Line) -> bool {
        let v1 = self.p2 - self.p1;
        let v2 = other.p2 - other.p1;
        return v1.cross(v2).abs() <= 1e-8;
    }

    #[must_use]
    /// Retorna se uma reta intersecta a outra
    pub fn intersects(&self, other: Line) -> bool {
        let ab = self.p2 - self.p1;
        let ac = other.p1 - self.p1;
        let ad = other.p2 - self.p1;
        let s1 = ab.cross(ac);
        let s2 = ab.cross(ad);

        // Se s1 e s2 tem o mesmo sinal, C e D estão no mesmo lado de AB.
        // Logo, não há interseção.
        if s1 * s2 >= 0.0 {
            return false;
        }

        let cd = other.p2 - other.p1;
        let ca = self.p1 - other.p1;
        let cb = self.p2 - other.p1;
        let s3 = cd.cross(ca);
        let s4 = cd.cross(cb);

        // Se s3 e s4 não tem o mesmo sinal, A e B não estão no mesmo lado de CD.
        // Logo, há interseção.
        return s3 * s4 < 0.0;
    }

    #[must_use]
    pub fn direction(&self) -> Vec2 {
        (self.p2 - self.p1).normalized()
    }

    #[must_use]
    /// Retorna a interseção entre duas retas (None se não há interseção).
    /// Utiliza a equação de interseção entre reta e hiperplano.
    pub fn intersection(&self, other: Line) -> Option<Intersection> {
        let r = self.p2 - self.p1;
        let s = other.p2 - other.p1;
        let n = Vec2::new(-s.y, s.x);

        // Se o produto escalar entre a reta e a normal da outra reta é igual a 0,
        // isso quer dizer que elas são paralelas e não há colisão.
        let bottom = r.dot(n);
        if bottom.abs() < 1e-8 {
            return None;
        }

        // t da interseção da reta1 com a reta2 (colisão da função raio)
        let t = (other.p1 - self.p1).cross(s) / r.cross(s);
        // "t" da interseção da reta2 com a reta1 (colisão da função raio)
        let u = (other.p1 - self.p1).cross(r) / r.cross(s);

        Some(Intersection {
            t,
            u,
            p: self.p1 + r * t,
            normal: other.normal(),
        })
    }
}
