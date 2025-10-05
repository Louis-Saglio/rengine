use rengine::build_variant::{BENCHMARK_BV, BUILD_VARIANT, DEMO_BV, FRAMEBUFFER_BENCHMARK_BV, FRAMEBUFFER_BV, TEST_BV, TEST_FRAMEBUFFER_BV};
use rengine::graphical_engine;
use rengine::physics::Particle;
use std::time::Instant;

use rengine::framebuffer;
use rengine::raw_engine;
use rengine::test_framebuffer::test_framebuffer;

fn main() {
    if BUILD_VARIANT == BENCHMARK_BV {
        raw_engine::run(Particle::new_random_pop_in_screen(1920, 1080));
    } else if BUILD_VARIANT == TEST_BV || BUILD_VARIANT == DEMO_BV {
        graphical_engine::run();
    } else if BUILD_VARIANT == FRAMEBUFFER_BV {
        framebuffer::run();
    } else if BUILD_VARIANT == FRAMEBUFFER_BENCHMARK_BV {
        let start = Instant::now();
        framebuffer::run();
        println!("Framebuffer execution time: {:?}", start.elapsed());
    } else if BUILD_VARIANT == TEST_FRAMEBUFFER_BV {
        test_framebuffer()
    } else {
        println!("Unknown build variant '{BUILD_VARIANT}'")
    }
}
