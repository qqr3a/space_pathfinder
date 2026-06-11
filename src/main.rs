#![allow(non_snake_case)]

mod physics;
mod renderer;
mod simulation;

#[macroquad::main("MyGame")]

async fn main() {
    let mut simulation = simulation::Simulation::new(60.0);
    let renderer = renderer::Renderer::new();
    simulation.setOrbits();

    loop {
        simulation.update();
        renderer.frame(&simulation).await;
    }
}
