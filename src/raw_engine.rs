use std::time::Instant;
use load_env_var_as::get_iterations_from_env_var;
use crate::physics::{apply_force, Population};

const ITERATIONS: u32 = get_iterations_from_env_var!();

pub fn run(particles: Population) {
    let mut particles = particles;
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        particles = apply_force(&particles);
    }
    let duration = start.elapsed();
    println!("Total time elapsed is: {:?}", duration);
    if ITERATIONS > 0 {
        println!("Microsec per update: {:?}", duration.as_micros() / ITERATIONS as u128);
    }
    if duration.as_millis() > 0 {
        println!("UPS: {:?}", (ITERATIONS * 1000) / duration.as_millis() as u32);
    }
}
