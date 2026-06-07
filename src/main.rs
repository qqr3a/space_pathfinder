mod body;

use body::Body;
use body::Vector2D;

fn main() {
    let mut earth = Body::new("earth", 2.0, Vector2D::new(5.0, 3.0));
    let mut moon = Body::new("moon", 1.0, Vector2D::new(3.0, 2.0));
    let mut entities = [earth, moon];

    for _n in 1..=2 {
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let (left, right) = entities.split_at_mut(j);
                let body1 = &mut left[i];
                let body2 = &mut right[0];

                body1.apply_gravitational_forces(body2);
            }
        }

        for body in &mut entities {
            body.print();
            body.update(1.0);
        }
    }
}
