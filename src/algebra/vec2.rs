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

/// Epsilon para erro numérico
const EPS: f64 = 1e-6;
const EPS_SQR: f64 = EPS * EPS;

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
    /// Checa se o tamanho do vetor é 1 (threshold: `1e-6` aplicado ao comprimento ao quadrado -> `1e-12`)
    pub fn is_normalized(self) -> bool {
        return (self.length_squared() - 1.0).abs() <= EPS_SQR;
    }

    #[inline]
    #[must_use]
    /// Checa se o vetor é paralelo ao outro (threshold: `1e-6`)
    pub fn is_parallel(self, other: Vec2) -> bool {
        return self.cross(other).abs() <= EPS;
    }

    #[inline]
    #[must_use]
    /// Checa se o vetor é igual ao outro com threshold: `1e-6`
    /// (aplicado ao comprimento ao quadrado -> `1e-12`)
    pub fn is_same(self, other: Vec2) -> bool {
        return self.distance_to_squared(other) <= EPS_SQR;
    }

    #[inline]
    #[must_use]
    /// Usa a definição do produto escalar para calcular o ângulo entre dois vetores
    pub fn angle_between(self, other: Vec2) -> f64 {
        let cos_theta = self.normalized().dot(other.normalized()).clamp(-1.0, 1.0);
        return cos_theta.acos();
    }

    #[inline]
    #[must_use]
    /// Usa a definição do produto vetorial para calcular o ângulo entre dois vetores
    pub fn angle_between_cross(self, other: Vec2) -> f64 {
        let sin_theta = self.normalized().cross(other.normalized()).clamp(-1.0, 1.0);
        return sin_theta.asin();
    }

    #[inline]
    #[must_use]
    /// Usa a fórmula `1 - (u.v / ||u||||v||)` para retornar o pseudoângulo do cosseno
    pub fn cos_pseudoangle_between(self, other: Vec2) -> f64 {
        let top = self.dot(other);
        let bottom = self.length() * other.length();
        return 1.0 - (top / bottom).clamp(-1.0, 1.0);
    }

    #[inline]
    #[must_use]
    /// Retorna o pseudoângulo em `[0,8)` do vetor no perímetro do quadrado
    pub fn square_pseudoangle(self) -> f64 {
        let x = self.x;
        let y = self.y;

        // Valores absolutos para evitar problemas com coordenadas negativas
        let ax = x.abs();
        let ay = y.abs();

        if x >= 0.0 {
            if y >= 0.0 {
                // Octantes 1 e 2
                if ax >= ay {
                    // 1
                    ay / ax
                } else {
                    // 2 -> px é "excesso" de (0,1)
                    2.0 - ax / ay
                }
            } else {
                // Octantes 7 e 8
                if ax >= ay {
                    // 8 -> px é "excesso" de (1,0)
                    8.0 - ay / ax
                } else {
                    // 7 -> ponto (0,-1) vale 6.0, soma px
                    6.0 + ax / ay
                }
            }
        } else {
            if y >= 0.0 {
                // Octantes 3 e 4
                if ax >= ay {
                    // 4 -> px é "excesso" de (-1,0)
                    4.0 - ay / ax
                } else {
                    // 3 -> ponto (0,1) vale 2.0, soma px
                    2.0 + ax / ay
                }
            } else {
                // Octantes 5 e 6
                if ax >= ay {
                    // 5 -> ponto (-1,0) vale 4.0, soma px
                    4.0 + ay / ax
                } else {
                    // 6 -> px é "excesso" de (0,-1)
                    6.0 - ax / ay
                }
            }
        }
    }

    #[inline]
    #[must_use]
    /// Retorna o pseudoângulo entre dois vetores em `[0,8)`
    pub fn square_pseudoangle_between(self, other: Vec2) -> f64 {
        let a = self.square_pseudoangle();
        let b = other.square_pseudoangle();

        let pseudo = b - a;
        if pseudo < 0.0 {
            // mantém o range em [0,8)
            pseudo + 8.0
        } else {
            pseudo
        }
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
