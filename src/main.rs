#![allow(non_snake_case)]
mod physics;
use std::time::{Duration, Instant};

use physics::body::Body;

const TICK_RATE: f64 = 60.0;
const TICK_STEP: f64 = 1.0 / TICK_RATE;

fn main() {
    let earth = Body::new("earth", 5.972e24, 0.0);
    let moon = Body::new("moon", 7.348e22, 385000000.0);
    let mut entities = [earth, moon];

    let TICK_DURATION = Duration::from_secs_f64(TICK_STEP);

    let mut accumulator = Duration::ZERO;
    let mut previousFrameTime = Instant::now();

    loop {
        let now = Instant::now();
        let frameTime = now - previousFrameTime;
        previousFrameTime = now;
        accumulator += frameTime;

        while accumulator >= TICK_DURATION {
            for i in 0..entities.len() {
                for j in (i + 1)..entities.len() {
                    let (left, right) = entities.split_at_mut(j);
                    let body1 = &mut left[i];
                    let body2 = &mut right[0];

                    body1.update_gravitational_forces(body2);
                }
            }

            for body in &mut entities {
                body.print();
                body.update(TICK_STEP);
            }

            accumulator -= TICK_DURATION;
        }

        // render frame would go here
        //
    }
}
