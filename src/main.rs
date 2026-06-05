struct Vector2D {
    x: i32,
    y: i32,
}

impl Vector2D {
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

    let mut test_vector1 = Vector2D { x: 1, y: 3 };
    let mut test_vector2 = Vector2D { x: 1, y: 5 };

    (&test_vector2 + &test_vector1).print();
    test_vector1.print();
    test_vector2.print();

    test_vector2 += &test_vector1;
    test_vector2.print();
    earth.print();
}
