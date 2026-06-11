#![allow(non_snake_case)]
use macroquad::prelude::*;

mod physics;
use std::time::{Duration, Instant};

use physics::body::Body;

const TICK_RATE: f64 = 60.0;
const TICK_STEP: f64 = 1.0 / TICK_RATE;
use macroquad::prelude::*;

#[macroquad::main("MyGame")]

async fn main() {
    let earth = Body::new("earth", 5.972e24, 0.0);
    let moon = Body::new("moon", 7.348e22, 385000000.0);
    let mut entities = [earth, moon];

    let TICK_DURATION = Duration::from_secs_f64(TICK_STEP);

    let mut accumulator = Duration::ZERO;
    let mut previousFrameTime = Instant::now();

    let mut timeSinceLastLog = Duration::ZERO;
    let logInterval = Duration::from_secs(1);

    loop {
        let now = Instant::now();
        let frameTime = now - previousFrameTime;
        previousFrameTime = now;
        accumulator += frameTime;
        timeSinceLastLog += frameTime;

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
                body.update(TICK_STEP * 3600.0);
            }

            accumulator -= TICK_DURATION;
        }

        if timeSinceLastLog >= logInterval {
            timeSinceLastLog -= logInterval;
            println!("---");
            for body in &mut entities {
                body.print();
            }
        }
        // render frame would go here
        //

        clear_background(BLACK);

        let scale = 1e6;

        draw_circle(
            screen_width() / 2.0 - (entities[0].getPositionX() / scale) as f32,
            screen_height() / 2.0 - (entities[0].getPositionY() / scale) as f32,
            15.0,
            BLUE,
        );
        draw_circle(
            screen_width() / 2.0 - (entities[1].getPositionX() / scale) as f32,
            screen_height() / 2.0 - (entities[1].getPositionY() / scale) as f32,
            5.0,
            GRAY,
        );

        next_frame().await
    }
}
