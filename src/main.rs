use rengine::build_variant::{BENCHMARK_BV, BENCHMARK_MULTI_THREAD_BV, BUILD_VARIANT, DEMO_BV, TEST_BV};
use rengine::physics::{Particle, DIMENSIONS, G, MINIMAL_DISTANCE, POP_SIZE};
use rengine::{multi_threaded_raw_engine, raw_engine};

fn main() {
    use rengine::graphical_engine;
    println!("DIMENSIONS: {DIMENSIONS}\nPOP_SIZE: {POP_SIZE}\nG: {G}\nMINIMAL_DISTANCE: {MINIMAL_DISTANCE}\nWORKER_NBR: {WORKER_NBR}");
    if BUILD_VARIANT == BENCHMARK_MULTI_THREAD_BV {
        multi_threaded_raw_engine::run(10000);
    } else if BUILD_VARIANT == BENCHMARK_BV {
        raw_engine::run(Particle::new_random_pop(), 10000);
    } else if BUILD_VARIANT == TEST_BV || BUILD_VARIANT == DEMO_BV {
        graphical_engine::run();
    } else {
        println!("Unknown build variant '{BUILD_VARIANT}'")
    }
}
