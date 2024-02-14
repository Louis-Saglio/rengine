use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread::spawn;
use crate::physics::{apply_force_with_workers, compute_acceleration_in_worker, Particle, WORKER_NBR};

pub fn run() {
    let particles = Particle::new_random_pop();
    let (sender_to_workers, receiver_from_main_thread) = channel();
    let (sender_to_main_thread, receiver_from_from_workers) = channel();
    let receiver_from_main_thread = Arc::new(Mutex::new(receiver_from_main_thread));
    let mut workers = Vec::new();
    for _ in 0..WORKER_NBR {
        let receiver_from_main_thread = Arc::clone(&receiver_from_main_thread);
        let sender_to_main_thread = sender_to_main_thread.clone();
        workers.push(spawn(move || compute_acceleration_in_worker(&particles, receiver_from_main_thread, sender_to_main_thread)))
    }
    for i in 0..10000 {
        apply_force_with_workers(&particles, &sender_to_workers, &receiver_from_from_workers);
    }
}