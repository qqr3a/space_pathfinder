pub struct Vector2D {
    x: i32,
    y: i32,
}

impl Vector2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn print(&self) {
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
impl Div<u32> for Vector2D {
    type Output = Vector2D;
    fn div(self, divident: u32) -> Vector2D {
        Vector2D {
            x: self.x / divident as i32,
            y: self.y / divident as i32,
        }
    }
}

pub struct Body {
    mass: u32,
    position: Vector2D,
    velocity: Vector2D,
    acceleration: Vector2D,
}

impl Body {
    pub fn new(mass: u32, position: Vector2D) -> Self {
        Self {
            mass,
            position,
            velocity: Vector2D::new(0, 0),
            acceleration: Vector2D::new(0, 0),
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

    pub fn applyForce(&mut self, force: Vector2D) {
        self.acceleration += &(force / self.mass);
    }
}
