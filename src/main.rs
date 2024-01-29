use rengine::physics::{DIMENSIONS, POP_SIZE, G, MINIMAL_DISTANCE, WORKER_NBR};

fn main() {
    use rengine::graphical_engine;
    println!("DIMENSIONS: {DIMENSIONS}\nPOP_SIZE: {POP_SIZE}\nG: {G}\nMINIMAL_DISTANCE: {MINIMAL_DISTANCE}\nWORKER_NBR: {WORKER_NBR}");
    graphical_engine::run();
}

fn main() {
    use rengine::raw_engine;
    use rengine::physics::Particle;
    println!("DIMENSIONS: {DIMENSIONS}\nPOP_SIZE: {POP_SIZE}\nG: {G}\nMINIMAL_DISTANCE: {MINIMAL_DISTANCE}\nWORKER_NBR: {WORKER_NBR}");
    raw_engine::run(Particle::new_random_pop(), 10000);
}
