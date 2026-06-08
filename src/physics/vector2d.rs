pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn print(&self) {
        println!("({}, {})", self.x, self.y)
    }
}

use std::ops::Add;
impl Add for &Vector2D {
    type Output = Vector2D;
    fn add(self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

use std::ops::AddAssign;
impl AddAssign<&Vector2D> for Vector2D {
    fn add_assign(&mut self, other: &Vector2D) {
        self.x += other.x;
        self.y += other.y;
    }
}
use std::ops::Div;
impl Div<f64> for Vector2D {
    type Output = Vector2D;
    fn div(self, divident: f64) -> Vector2D {
        Vector2D {
            x: self.x / divident as f64,
            y: self.y / divident as f64,
        }
    }
}

use std::ops::Sub;
impl Sub for &Vector2D {
    type Output = Vector2D;
    fn sub(self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

use std::ops::Mul;
impl Mul<f64> for &Vector2D {
    type Output = Vector2D;
    fn mul(self, scaler: f64) -> Vector2D {
        Vector2D {
            x: self.x * scaler as f64,
            y: self.y * scaler as f64,
        }
    }
}

impl Mul<&Vector2D> for &Vector2D {
    type Output = f64;
    fn mul(self, other: &Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
}
