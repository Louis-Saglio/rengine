use rengine::physics::Particle;
use std::time::Instant;

use rengine::framebuffer;
use rengine::raw_engine;

const BUILD_VARIANT: Option<&str> = option_env!("BUILD_VARIANT");

fn main() {
    match BUILD_VARIANT {
        None => println!("Build variant not specified"),
        Some(build_variant) => match build_variant {
            "BENCHMARK" => raw_engine::run(Particle::new_random_pop_in_screen(2560, 1440)),
            "FRAMEBUFFER" => framebuffer::run(),
            "FRAMEBUFFER_BENCHMARK" => {
                let start = Instant::now();
                framebuffer::run();
                println!("Framebuffer execution time: {:?}", start.elapsed());
            }
            _ => println!("Unknown build variant '{build_variant}'"),
        },
    }
}
