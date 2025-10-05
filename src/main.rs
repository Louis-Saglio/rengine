use rengine::physics::Particle;

use rengine::framebuffer;
use rengine::raw_engine;

const BUILD_VARIANT: Option<&str> = option_env!("BUILD_VARIANT");

fn main() {
    match BUILD_VARIANT {
        None => println!("Build variant not specified"),
        Some(build_variant) => match build_variant {
            "BENCHMARK" => raw_engine::run(Particle::new_random_pop_in_screen(2560, 1440)),
            "FRAMEBUFFER" => {
                framebuffer::run(&mut Particle::new_random_pop_in_screen(2560, 1440));
            }
            "TEST" => {
                let mut pop = Particle::new_test_pop();
                framebuffer::run(&mut pop);
                assert_eq!(
                    pop,
                    [
                        Particle {
                            mass: 15.0,
                            speed: [-0.6309910380664001, -1.8054294207312598],
                            position: [-131.04483937204387, -426.65005151876125]
                        },
                        Particle {
                            mass: 10.0,
                            speed: [0.6780727901268769, 2.2440584293094075],
                            position: [59.958335444060374, 189.0186798571582]
                        },
                        Particle {
                            mass: 10.0,
                            speed: [0.26841376697272246, 0.4640857017874784],
                            position: [286.6089236140069, 400.95639742098393]
                        }
                    ]
                )
            }
            _ => println!("Unknown build variant '{build_variant}'"),
        },
    }
}
