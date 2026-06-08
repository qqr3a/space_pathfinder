mod body;

use body::Body;
use body::Vector2D;

fn main() {
    let mut earth = Body::new("earth", 5.972*(10.0 as f64).powf(24 as f64), Vector2D::new(0.0, 0.0));
    let mut moon = Body::new("moon", 7.348*(10.0 as f64).powf(22 as f64), Vector2D::new(385000000.0, 0.0));
    let mut entities = [earth, moon];

    for _n in 1..=1000 {
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
