use rengine::graphical_engine;
use rengine::physics::Particle;
use rengine::raw_engine;

fn main() {
    graphical_engine::run();
    // raw_engine::run(Particle::new_random_pop(), 10000);
}
