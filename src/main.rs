mod body;
use body::Body;
use body::Vector2D;

fn main() {
    let earth = Body::new(2, Vector2D::new(2, 4));

    earth.print();
}
