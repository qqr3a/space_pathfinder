use crate::simulation;
use macroquad::prelude::*;
use simulation::Simulation;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn frame(&self, simulation: &Simulation) {
        clear_background(BLACK);

        let scale = 1e6;

        draw_circle(
            screen_width() / 2.0 - (simulation.entities[0].getPositionX() / scale) as f32,
            screen_height() / 2.0 - (simulation.entities[0].getPositionY() / scale) as f32,
            (simulation.entities[0].getRadius() / scale) as f32 + 1.0,
            BLUE,
        );
        draw_circle(
            screen_width() / 2.0 - (simulation.entities[1].getPositionX() / scale) as f32,
            screen_height() / 2.0 - (simulation.entities[1].getPositionY() / scale) as f32,
            (simulation.entities[1].getRadius() / scale) as f32 + 1.0,
            GRAY,
        );

        next_frame().await;
    }
}
