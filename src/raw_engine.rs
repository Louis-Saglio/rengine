use crate::physics::{Population, apply_force};
use load_env_var_as::get_iterations_from_env_var;
use std::time::Instant;

const ITERATIONS: u32 = get_iterations_from_env_var!();

pub fn run(population: Population) {
    let mut population = population;
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        apply_force(&mut population);
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
