use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const NULL: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const X: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const Y: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    #[inline]
    /// Vec2's constructor.
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    #[inline]
    /// Returns the dot product between two vectors
    pub fn dot(self, rhs: Vec2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    /// Returns the cross product between two vectors
    pub fn cross(self, rhs: Vec2) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    #[inline]
    /// Returns the square of the magnitude of the vector. Faster than magnitude() * magnitude().
    pub fn magnitude_squared(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    /// Returns the magnitude of the vector
    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    #[inline]
    /// Returns a normalized version of the vector (divides the vector by its length)
    pub fn normalized(self) -> Vec2 {
        self / self.magnitude()
    }

    #[inline]
    /// Checks if the vector's magnitude equals 1 (within a certain threshold: `1e-8`)
    pub fn is_normalized(self) -> bool {
        (self.magnitude_squared() - 1.0).abs() <= 1e-8
    }

    #[inline]
    /// Uses the dot product definition to return the angle between two vectors
    pub fn angle_between(self, other: Vec2) -> f64 {
        let cos_theta = self.normalized().dot(other.normalized());
        cos_theta.acos()
    }

    #[inline]
    /// Rotates the vector by a certain angle.
    /// This uses the rotation matrix definition for the calculations.
    pub fn rotated(self, angle: f64) -> Vec2 {
        let new_x = self.x * angle.cos() - self.y * angle.sin();
        let new_y = self.x * angle.sin() + self.y * angle.cos();
        Vec2 { x: new_x, y: new_y }
    }
}

////// OPERATOR OVERLOADS //////
/// Vector addition
impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Vector subtraction
impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Division by scalar
impl Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

/// Multiplication by scalar
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Negative of vector - Short for `vec * -1`
impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// Implements display trait for better printing
impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
