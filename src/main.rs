use rengine::physics::{Coordinates, Particle};

#[cfg(any(feature = "framebuffer", feature = "e2e-test"))]
use rengine::framebuffer;
#[cfg(feature = "benchmark")]
use rengine::raw_engine;

#[cfg(feature = "benchmark")]
fn main() {
    raw_engine::run(Particle::new_random_pop_in_screen(2560, 1440));
}

#[cfg(feature = "framebuffer")]
fn main() {
    framebuffer::run(&mut Particle::new_random_pop_in_screen(2560, 1440));
}

#[cfg(feature = "e2e-test")]
fn main() {
    let mut pop = Particle::new_test_pop();
    framebuffer::run(&mut pop);
    assert_eq!(
        pop[..3],
        [
            Particle {
                mass: 15.0,
                speed: Coordinates::new([-0.6309910380664001, -1.8054294207312598]),
                position: Coordinates::new([-131.04483937204387, -426.65005151876125])
            },
            Particle {
                mass: 10.0,
                speed: Coordinates::new([0.6780727901268769, 2.2440584293094075]),
                position: Coordinates::new([59.958335444060374, 189.0186798571582])
            },
            Particle {
                mass: 10.0,
                speed: Coordinates::new([0.26841376697272246, 0.4640857017874784]),
                position: Coordinates::new([286.6089236140069, 400.95639742098393])
            }
        ]
    )
}

#[cfg(not(any(feature = "benchmark", feature = "framebuffer", feature = "e2e-test")))]
fn main() {
    compile_error!("No build variant selected. Use --features benchmark, framebuffer, or e2e-test");
}
