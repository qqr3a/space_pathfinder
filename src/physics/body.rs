use super::vector2d::Vector2D;

const G: f64 = 6.6743015e-11;

#[derive(Clone)]
pub struct Body {
    name: String,
    mass: f64,
    radius: f64,
    position: Vector2D,
    velocity: Vector2D,
    force: Vector2D,
}

impl Body {
    pub fn new(name: impl Into<String>, mass: f64, radius: f64, x_position: f64) -> Self {
        Self {
            name: name.into(),
            mass,
            radius,
            position: Vector2D::new(x_position, 0.0),
            velocity: Vector2D::new(0.0, 0.0),
            force: Vector2D::new(0.0, 0.0),
        }
    }

    pub fn print(&self) {
        println!("Name: {}", self.name);
        println!("Mass: {}", self.mass);
        print!("Position: ");
        self.position.print();
        print!("Velocity: ");
        self.velocity.print();
        let acceleration = self.force / self.mass;
        print!("Acceleration: ");
        acceleration.print();
    }

    pub fn update(&mut self, deltaTime: f64) {
        let acceleration = self.force / self.mass;
        self.force.reset();
        self.velocity += acceleration * deltaTime;

        self.position += self.velocity * deltaTime;
    }

    fn apply_force(&mut self, force: Vector2D) {
        self.force += force;
    }

    pub fn update_gravitational_forces(&mut self, body2: &mut Body) {
        let displacement = body2.position - self.position;
        let r_squared = displacement * displacement;
        let r = r_squared.sqrt();
        let scaler = G * self.mass * body2.mass / (r_squared * r); // instead of r_squared^ (3/2), r * r_squared is the same

        let force = displacement * scaler;

        self.apply_force(force);
        body2.apply_force(-force);
    }

    pub fn setOrbitVelocity(&mut self, body2: &Body) {
        let displacement = body2.position - self.position;
        let r_squared = displacement * displacement;
        let r = r_squared.sqrt();
        self.velocity.y = (G * body2.mass / r).sqrt();
    }

    pub fn getPositionX(&self) -> f64 {
        self.position.x
    }
    pub fn getPositionY(&self) -> f64 {
        self.position.y
    }
    pub fn getRadius(&self) -> f64 {
        self.radius
    }
}
