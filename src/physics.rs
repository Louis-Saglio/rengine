// Responsible for defining newtonian physic

use load_env_var_as::{
    get_default_particle_mass_from_env_var, get_dimensions_from_env_var, get_g_from_env_var,
    get_minimal_distance_from_env_var, get_pop_size_from_env_var,
};
use rand::Rng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::array;
use std::sync::Mutex;

pub const DIMENSIONS: usize = get_dimensions_from_env_var!();

type Coordinates = [f64; DIMENSIONS];

const DEFAULT_COORDINATES: Coordinates = [0f64; DIMENSIONS];

pub const POP_SIZE: usize = get_pop_size_from_env_var!();

pub const G: f64 = get_g_from_env_var!();

pub const MINIMAL_DISTANCE: f64 = get_minimal_distance_from_env_var!();
const MINIMAL_DISTANCE_SQUARED: f64 = MINIMAL_DISTANCE * MINIMAL_DISTANCE;

const DEFAULT_PARTICLE: Particle = Particle {
    mass: 0f64,
    speed: DEFAULT_COORDINATES,
    position: DEFAULT_COORDINATES,
};

pub type Population = [Particle; POP_SIZE];

const DEFAULT_POP: Population = [DEFAULT_PARTICLE; POP_SIZE];

#[derive(Clone, Copy, Default, Debug)]
pub struct Particle {
    pub mass: f64,
    pub speed: Coordinates,
    pub position: Coordinates,
}

impl Particle {
    fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            mass: 1f64,
            speed: [0f64; DIMENSIONS],
            position: rng.gen(),
        }
    }

    pub fn new_random_pop() -> Population {
        let mut pop = DEFAULT_POP;
        for slot in pop.iter_mut() {
            *slot = Self::new_random();
        }
        pop
    }

    pub fn new_random_pop_in_screen(width: u32, height: u32) -> Population {
        let mut rng = rand::thread_rng();
        let mut pop = DEFAULT_POP;
        let half_width = width as f64 / 2f64;
        let half_height = height as f64 / 2f64;
        for i in 0..POP_SIZE {
            let mut position = DEFAULT_COORDINATES;
            position[0] = rng.gen_range((-half_width)..half_width);
            position[1] = rng.gen_range((-half_height)..half_height);
            for i in 2..DIMENSIONS {
                position[i] = rng.gen_range(-100.0..100.0);
            }
            pop[i] = Self {
                mass: get_default_particle_mass_from_env_var!(),
                speed: DEFAULT_COORDINATES,
                position,
            };
        }
        pop
    }

    pub fn new_test_pop() -> Population {
        let mut pop = DEFAULT_POP.clone();
        if POP_SIZE < 3 {
            panic!("POP_SIZE must be 3 for test")
        } else if DIMENSIONS != 2 {
            panic!("DIMENSIONS must be 2 for test")
        } else {
            pop[0] = Particle {
                mass: 15f64,
                speed: DEFAULT_COORDINATES,
                position: {
                    let mut position = DEFAULT_COORDINATES;
                    position[0] = 100f64;
                    position[1] = 100f64;
                    position
                },
            };
            pop[1] = Particle {
                mass: 10f64,
                speed: DEFAULT_COORDINATES,
                position: {
                    let mut position = DEFAULT_COORDINATES;
                    position[0] = 100f64;
                    position[1] = -100f64;
                    position
                },
            };
            pop[2] = Particle {
                mass: 10f64,
                speed: DEFAULT_COORDINATES,
                position: {
                    let mut position = DEFAULT_COORDINATES;
                    position[0] = -100f64;
                    position[1] = -100f64;
                    position
                },
            }
        }
        pop
    }
}

pub fn distance_squared(a: Coordinates, b: Coordinates) -> f64 {
    if DIMENSIONS == 2 {
        let diff0 = a[0] - b[0];
        let diff1 = a[1] - b[1];
        diff0 * diff0 + diff1 * diff1
    } else if DIMENSIONS == 3 {
        let diff0 = a[0] - b[0];
        let diff1 = a[1] - b[1];
        let diff2 = a[2] - b[2];
        diff0 * diff0 + diff1 * diff1 + diff2 * diff2
    } else if DIMENSIONS == 4 {
        let diff0 = a[0] - b[0];
        let diff1 = a[1] - b[1];
        let diff2 = a[2] - b[2];
        let diff3 = a[3] - b[3];
        diff0 * diff0 + diff1 * diff1 + diff2 * diff2 + diff3 * diff3
    } else if DIMENSIONS == 5 {
        let diff0 = a[0] - b[0];
        let diff1 = a[1] - b[1];
        let diff2 = a[2] - b[2];
        let diff3 = a[3] - b[3];
        let diff4 = a[4] - b[4];
        diff0 * diff0 + diff1 * diff1 + diff2 * diff2 + diff3 * diff3 + diff4 * diff4
    } else {
        let mut sum = 0.0;
        for i in 0..DIMENSIONS {
            let diff = a[i] - b[i];
            sum += diff * diff;
        }
        sum
    }
}

pub struct ApplyForceContext {
    pub population: Population,
}

pub fn apply_force(context: &mut ApplyForceContext) {
    // We are going to mutate the particles stored in this array to register the changes in acceleration and speed
    // todo: should mutate the original, and use the copy as src for computations
    let mut computed_particles = context.population;
    let particles = &context.population;

    // This vector will contain the pairs of particles index to merge together.
    let mut to_merge = Vec::new();
    let to_merge_mutex: Mutex<&mut Vec<(usize, usize)>> = Mutex::new(&mut to_merge);

    computed_particles
        .par_iter_mut()
        .zip(0..POP_SIZE)
        .for_each(|(computed_particle_a, particle_a_index)| {
            let particle_a = &particles[particle_a_index];

            if particle_a.mass != 0f64 {
                // If a particle has no mass it is exactly like it does not exist
                for particle_b_index in 0..POP_SIZE {
                    let particle_b = &particles[particle_b_index];
                    // If a particle has no mass it is exactly like it does not exist
                    if particle_b.mass == 0f64 || particle_a_index == particle_b_index {
                        continue;
                    }

                    let distance_squared = distance_squared(particle_a.position, particle_b.position);

                    // These variables may seem esoteric, but they were set up because benchmarks showed that
                    // they provided better performances than more natural choices
                    let g_by_d_squared = G / (distance_squared);
                    let inverse_distance_square_root = 1f64 / distance_squared.sqrt();
                    let force_by_mass_a = particle_b.mass * g_by_d_squared * inverse_distance_square_root;

                    // Accelerate the two particles in all dimensions
                    for i in 0..DIMENSIONS {
                        let direction = particle_b.position[i] - particle_a.position[i];
                        computed_particle_a.speed[i] += direction * force_by_mass_a;
                    }

                    if distance_squared < MINIMAL_DISTANCE_SQUARED {
                        let mut to_merge = to_merge_mutex
                            .lock()
                            .expect("Critical unrecoverable failure when registering particles to merge");
                        to_merge.push((particle_a_index, particle_b_index));
                    }
                }
            }
            // Move particle based on its speed during the previous frame
            for i in 0..DIMENSIONS {
                computed_particle_a.position[i] += particle_a.speed[i];
            }
        });

    for (particle_a_index, particle_b_index) in to_merge.iter() {
        let particle_a = computed_particles[*particle_a_index];
        let particle_b = computed_particles[*particle_b_index];
        if particle_a.mass == 0f64 || particle_b.mass == 0f64 {
            continue;
        }
        let (index_to_fuse, index_to_delete) = if particle_a.mass > particle_b.mass {
            (*particle_a_index, *particle_b_index)
        } else {
            (*particle_b_index, *particle_a_index)
        };
        computed_particles[index_to_delete].mass = 0f64;
        computed_particles[index_to_fuse].mass = particle_a.mass + particle_b.mass;
        computed_particles[index_to_fuse].position = array::from_fn(|i| {
            (particle_a.position[i] * particle_a.mass + particle_b.position[i] * particle_b.mass)
                / (particle_a.mass + particle_b.mass)
        });
        computed_particles[index_to_fuse].speed = array::from_fn(|i| {
            (particle_a.speed[i] * particle_a.mass + particle_b.speed[i] * particle_b.mass)
                / (particle_a.mass + particle_b.mass)
        });
    }

    context.population = computed_particles;
}

#[cfg(test)]
pub mod test {
    use crate::physics::{apply_force, Particle, Population};

    #[test]
    fn test_apply_force() {
        let mut population: Population = [
            Particle {
                mass: 3f64,
                speed: [0f64, 0f64],
                position: [10f64, 10f64],
            },
            Particle {
                mass: 2f64,
                speed: [0f64, 0f64],
                position: [-10f64, -10f64],
            },
            Particle {
                mass: 1f64,
                speed: [0f64, 0f64],
                position: [10f64, -10f64],
            },
        ];
        for _ in 0..100 {
            population = apply_force(&population);
        }
        assert_eq!(population[0].position, [-2.6124097114690477, -41.87865599101741])
    }
}
