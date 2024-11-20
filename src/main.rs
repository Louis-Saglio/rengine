use rengine::build_variant::{BENCHMARK_BV, BENCHMARK_MULTI_THREAD_BV, BUILD_VARIANT, DEMO_BV, FRAMEBUFFER_BV, TEST_BV};
use rengine::graphical_engine;
use rengine::physics::{Particle, DIMENSIONS, G, MINIMAL_DISTANCE, POP_SIZE};

use rengine::{multi_threaded_raw_engine, raw_engine};
use rengine::framebuffer::sandbox;

fn main() {
    if BUILD_VARIANT == BENCHMARK_MULTI_THREAD_BV {
        println!("DIMENSIONS: {DIMENSIONS}\nPOP_SIZE: {POP_SIZE}\nG: {G}\nMINIMAL_DISTANCE: {MINIMAL_DISTANCE}");
        multi_threaded_raw_engine::run(10000);
    } else if BUILD_VARIANT == BENCHMARK_BV {
        println!("DIMENSIONS: {DIMENSIONS}\nPOP_SIZE: {POP_SIZE}\nG: {G}\nMINIMAL_DISTANCE: {MINIMAL_DISTANCE}");
        raw_engine::run(Particle::new_random_pop(), 10000);
    } else if BUILD_VARIANT == TEST_BV || BUILD_VARIANT == DEMO_BV {
        println!("DIMENSIONS: {DIMENSIONS}\nPOP_SIZE: {POP_SIZE}\nG: {G}\nMINIMAL_DISTANCE: {MINIMAL_DISTANCE}");
        graphical_engine::run();
    } else if BUILD_VARIANT == FRAMEBUFFER_BV {
        sandbox();
    } else {
        println!("Unknown build variant '{BUILD_VARIANT}'")
    }
}
