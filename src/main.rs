use rengine::physics::Particle;
use rengine::graphical_engine;

fn main() {
    let particles = Particle::new_random_pop();
    graphical_engine::run(particles);
}
