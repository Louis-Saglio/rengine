use load_env_var_as_usize::get_worker_nbr_from_env_var;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::Instant;

use crate::physics::distributed::{apply_force_with_workers, compute_acceleration_in_worker};
use crate::physics::Particle;

const WORKER_NBR: usize = get_worker_nbr_from_env_var!();

pub fn run(iterations: u64) {
    let particles = Particle::new_random_pop();
    let start = Instant::now();

    let (sender_to_workers, receiver_from_main_thread) = channel();
    let (sender_to_main_thread, receiver_from_from_workers) = channel();

    let receiver_from_main_thread = Arc::new(Mutex::new(receiver_from_main_thread));

    let mut workers = Vec::new();
    for _ in 0..WORKER_NBR {
        let receiver_from_main_thread = receiver_from_main_thread.clone();
        let sender_to_main_thread = sender_to_main_thread.clone();
        workers.push(spawn(move || {
            compute_acceleration_in_worker(receiver_from_main_thread, sender_to_main_thread)
        }))
    }

    for iteration in 0..iterations {
        apply_force_with_workers(&particles, &sender_to_workers, &receiver_from_from_workers);
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
