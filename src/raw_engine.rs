use std::time::Instant;

use crate::physics::{apply_force, apply_force_multi_threaded, Population};

pub fn run(particles: Population, iterations: u64) {
    let mut particles = particles;
    let start = Instant::now();
    for iteration in 0..iterations {
        particles = apply_force(&particles);
        // particles = apply_force_multi_threaded(&particles);
        if iteration % 100 == 0 {
            println!("{iteration}")
        }
    }
    let duration = start.elapsed();
    println!("Total time elapsed is: {:?}", duration);
    println!("Microsec per update: {:?}", duration.as_micros() / iterations as u128);
    if duration.as_millis() > 0 {
        println!("UPS: {:?}", (iterations * 1000) / duration.as_millis() as u64);
    }
}
