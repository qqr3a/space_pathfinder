mod body;

use body::Body;
use body::Vector2D;

fn main() {
    let mut earth = Body::new(5.0, Vector2D::new(0.0, 0.0));

    earth.print();
    for _n in 1..=10 {
        earth.apply_force(Vector2D::new(1.0, 0.0));
        earth.update(1.0);
        earth.print();
    }
}
