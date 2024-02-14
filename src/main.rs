use rengine::build_variant::{BENCHMARK_BV, BUILD_VARIANT};
use rengine::physics::{Particle, DIMENSIONS, G, MINIMAL_DISTANCE, POP_SIZE, WORKER_NBR};
use rengine::{multi_threaded_engine, raw_engine, workers};

fn main() {
    use rengine::graphical_engine;
    println!("DIMENSIONS: {DIMENSIONS}\nPOP_SIZE: {POP_SIZE}\nG: {G}\nMINIMAL_DISTANCE: {MINIMAL_DISTANCE}\nWORKER_NBR: {WORKER_NBR}");
    if BUILD_VARIANT == BENCHMARK_BV {
        multi_threaded_engine::run()
    } else {
        graphical_engine::run();
    }
}
