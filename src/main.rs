struct Vector2D {
    x: i32,
    y: i32,
}

impl Vector2D {
    fn print(&self) {
        println!("({}, {})", self.x, self.y)
    }
}
struct Body {
    mass: u32,
    position: Vector2D,
}

impl Body {
    fn print(&self) {
        println!("Mass: {}", self.mass);
        print!("Position: ");
        self.position.print();
    }
}

fn main() {
    let earth = Body {
        mass: 2,
        position: Vector2D { x: 1, y: 2 },
    };
    earth.print();
}
