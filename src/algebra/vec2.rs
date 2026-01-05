use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use macroquad::color::Color;
use macroquad::shapes::draw_circle;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

// Métodos
impl Vec2 {
    pub const NULL: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const X: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const Y: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    #[inline]
    #[must_use]
    /// Construtor do Vec2.
    pub fn new(x: f64, y: f64) -> Vec2 {
        return Vec2 { x, y };
    }

    #[inline]
    #[must_use]
    /// Retorna o vetor unitário alinhado ao eixo X rotacionado em theta radianos
    pub fn from_angle(theta: f64) -> Vec2 {
        return Vec2::new(theta.cos(), theta.sin());
    }

    #[inline]
    pub fn draw(&self, color: Color) {
        draw_circle(self.x as f32, self.y as f32, 3.0, color);
    }

    #[inline]
    #[must_use]
    /// Retorna o produto escalar entre dois vetores
    pub fn dot(self, rhs: Vec2) -> f64 {
        return self.x * rhs.x + self.y * rhs.y;
    }

    #[inline]
    #[must_use]
    /// Retorna o produto vetorial entre dois vetores
    pub fn cross(self, rhs: Vec2) -> f64 {
        return self.x * rhs.y - self.y * rhs.x;
    }

    #[inline]
    #[must_use]
    /// Retorna o quadrado do tamanho do vetor (mais rápido que length() * length())
    pub fn length_squared(self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }

    #[inline]
    #[must_use]
    /// Retorna o tamanho do vetor
    pub fn length(self) -> f64 {
        return self.length_squared().sqrt();
    }

    #[inline]
    #[must_use]
    /// Retorna a distância ao quadrado de um ponto até o outro
    pub fn distance_to_squared(self, other: Vec2) -> f64 {
        (self - other).length_squared()
    }

    #[inline]
    #[must_use]
    /// Retorna a distância de um ponto até o outro
    pub fn distance_to(self, other: Vec2) -> f64 {
        (self - other).length()
    }

    #[inline]
    #[must_use]
    /// Retorna o vetor normalizado (divide o vetor pelo seu tamanho)
    pub fn normalized(self) -> Vec2 {
        return self / self.length();
    }

    #[inline]
    #[must_use]
    /// Checa se o tamanho do vetor é 1 (threshold: `1e-6`)
    pub fn is_normalized(self) -> bool {
        return (self.length_squared() - 1.0).abs() <= 1e-12;
    }

    #[inline]
    #[must_use]
    /// Checa se o vetor é paralelo ao outro (threshold: `1e-6`)
    pub fn is_parallel(self, other: Vec2) -> bool {
        return self.cross(other).abs() <= 1e-6;
    }

    #[inline]
    #[must_use]
    /// Usa a definição do produto escalar para calcular o ângulo entre dois vetores
    pub fn angle_between(self, other: Vec2) -> f64 {
        let cos_theta = self.normalized().dot(other.normalized());
        return cos_theta.acos();
    }

    #[inline]
    #[must_use]
    /// Gira o vetor em um ângulo específico ao redor da origem.
    /// Isso usa a definição da matriz de rotação para os cálculos.
    pub fn rotated(self, theta: f64) -> Vec2 {
        let new_x = self.x * theta.cos() - self.y * theta.sin();
        let new_y = self.x * theta.sin() + self.y * theta.cos();
        return Vec2 { x: new_x, y: new_y };
    }

    #[inline]
    #[must_use]
    /// Reflete o vetor em torno de um vetor normal
    pub fn bounce(self, normal: Vec2) -> Vec2 {
        self - 2.0 * self.dot(normal) * normal
    }
}

////// OPERATOR OVERLOADS //////
/// Adição de vetores
impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        return Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Subtração de vetores
impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        return Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Divisão por escalar
impl Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f64) -> Vec2 {
        return Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

/// Multiplicação por escalar
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Vec2 {
        return Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        return Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        };
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Negativo do vetor
impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        return Vec2 {
            x: -self.x,
            y: -self.y,
        };
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec2::NULL, |a, b| Vec2 {
            x: a.x + b.x,
            y: a.y + b.y,
        })
    }
}

/// Implementa o trait "Display" pra printar bonitinho etc.
impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
