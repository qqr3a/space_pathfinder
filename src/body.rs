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

pub struct Body {
    name: String,
    mass: f64,
    position: Vector2D,
    velocity: Vector2D,
    acceleration: Vector2D,
}

impl Body {
    pub fn new(name: impl Into<String>, mass: f64, position: Vector2D) -> Self {
        Self {
            name: name.into(),
            mass,
            position,
            velocity: Vector2D::new(0.0, 0.0),
            acceleration: Vector2D::new(0.0, 0.0),
        }
    }

    pub fn print(&self) {
        println!("Mass: {}", self.mass);
        print!("Position: ");
        self.position.print();
        print!("Velocity: ");
        self.velocity.print();
        print!("Acceleration: ");
        self.acceleration.print();
    }

    pub fn update(&mut self, deltaTime: f64) {
        self.velocity += &(&self.acceleration * deltaTime);
        self.acceleration = Vector2D::new(0.0, 0.0);
        self.position += &(&self.velocity * deltaTime);
    }

    fn apply_force(&mut self, force: Vector2D) {
        self.acceleration += &(force / self.mass);
    }

    pub fn apply_gravitational_forces(&mut self, body2: &mut Body) {
        let g = 1.0;

        let displacement = &body2.position - &self.position;
        let r_squared = &displacement * &displacement;
        let r = r_squared.sqrt();
        let scaler = g * self.mass * body2.mass / (r_squared * r); // instead of r_squared^ (3/2), r * r_squared is the same
        self.apply_force(&displacement * scaler);
        body2.apply_force(&displacement * -scaler);
    }
}
