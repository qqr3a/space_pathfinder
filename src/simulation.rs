use crate::physics::body::Body;
use std::time::{Duration, Instant};

pub struct Simulation {
    TICK_RATE: f64,
    TICK_STEP: f64,
    TICK_DURATION: Duration,
    pub entities: Vec<Body>,
    accumulator: Duration,
    previousFrameTime: Instant,
    timeSinceLastLog: Duration,
    logInterval: Duration,
}

impl Simulation {
    pub fn new(TICK_RATE: f64) -> Self {
        let TICK_STEP = 1.0 / TICK_RATE;
        Self {
            TICK_RATE,
            TICK_STEP,
            entities: vec![
                Body::new("earth", 5.972e24, 6371.0e3, 0.0),
                Body::new("moon", 7.348e22, 1737.5e3, 384_400.0e3),
            ],

            TICK_DURATION: Duration::from_secs_f64(TICK_STEP),
            accumulator: Duration::ZERO,
            previousFrameTime: Instant::now(),

            timeSinceLastLog: Duration::ZERO,
            logInterval: Duration::from_secs(1),
        }
    }

    pub fn setOrbits(&mut self) {
        let earth = self.entities[0].clone();
        self.entities[1].setOrbitVelocity(&earth);
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let frameTime = now - self.previousFrameTime;
        self.previousFrameTime = now;
        self.accumulator += frameTime;
        self.timeSinceLastLog += frameTime;

        while self.accumulator >= self.TICK_DURATION {
            self.tick();
            self.accumulator -= self.TICK_DURATION;
        }

        if self.timeSinceLastLog >= self.logInterval {
            self.timeSinceLastLog -= self.logInterval;
            println!("---");
            for body in &mut self.entities {
                body.print();
            }
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.entities.len() {
            for j in (i + 1)..self.entities.len() {
                let (left, right) = self.entities.split_at_mut(j);
                let body1 = &mut left[i];
                let body2 = &mut right[0];

                body1.update_gravitational_forces(body2);
            }
        }

        for body in &mut self.entities {
            body.update(self.TICK_STEP * 3600.0 * 24.0);
        }
    }
}
