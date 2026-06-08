use super::vector2d::Vector2D;

pub struct Body {
    name: String,
    mass: f64,
    position: Vector2D,
    velocity: Vector2D,
    acceleration: Vector2D,
    G: f64,
}

impl Body {
    pub fn new(name: impl Into<String>, mass: f64, x_position: f64) -> Self {
        Self {
            name: name.into(),
            mass,
            position: Vector2D::new(x_position, 0.0),
            velocity: Vector2D::new(0.0, 0.0),
            acceleration: Vector2D::new(0.0, 0.0),
            G: 6.6743015 * (10.0 as f64).powf(-11.0),
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
        let displacement = &body2.position - &self.position;
        let r_squared = &displacement * &displacement;
        let r = r_squared.sqrt();
        let scaler = self.G * self.mass * body2.mass / (r_squared * r); // instead of r_squared^ (3/2), r * r_squared is the same
        self.apply_force(&displacement * scaler);
        body2.apply_force(&displacement * -scaler);
    }
}
