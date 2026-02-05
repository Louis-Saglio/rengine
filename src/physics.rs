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

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Coordinates(pub [f64; DIMENSIONS]);

const DEFAULT_COORDINATES: Coordinates = Coordinates([0f64; DIMENSIONS]);

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

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Particle {
    pub mass: f64,
    pub speed: Coordinates,
    pub position: Coordinates,
}

impl Particle {
    fn new_random() -> Self {
        let mut rng = rand::rng();
        Self {
            mass: 1f64,
            speed: Coordinates([0f64; DIMENSIONS]),
            position: Coordinates(rng.random()),
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
        let mut rng = rand::rng();
        let mut pop = DEFAULT_POP;
        let half_width = width as f64 / 2f64;
        let half_height = height as f64 / 2f64;
        for slot in pop.iter_mut() {
            let mut position = DEFAULT_COORDINATES.0;
            position[0] = rng.random_range((-half_width)..half_width);
            position[1] = rng.random_range((-half_height)..half_height);
            for position in position.iter_mut().skip(2) {
                *position = rng.random_range(-100.0..100.0);
            }
            *slot = Self {
                mass: get_default_particle_mass_from_env_var!(),
                speed: DEFAULT_COORDINATES,
                position: Coordinates(position),
            };
        }
        pop
    }

    pub fn new_test_pop() -> Population {
        let mut pop = DEFAULT_POP;
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
                    position.0[0] = 100f64;
                    position.0[1] = 100f64;
                    position
                },
            };
            pop[1] = Particle {
                mass: 10f64,
                speed: DEFAULT_COORDINATES,
                position: {
                    let mut position = DEFAULT_COORDINATES;
                    position.0[0] = 100f64;
                    position.0[1] = -100f64;
                    position
                },
            };
            pop[2] = Particle {
                mass: 10f64,
                speed: DEFAULT_COORDINATES,
                position: {
                    let mut position = DEFAULT_COORDINATES;
                    position.0[0] = -100f64;
                    position.0[1] = -100f64;
                    position
                },
            }
        }
        pop
    }
}

pub fn distance_squared(Coordinates(a): Coordinates, Coordinates(b): Coordinates) -> f64 {
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

pub fn apply_force(population: &mut Population) {
    let previous_population = *population;

    // This vector will contain the pairs of particles index to merge together.
    let mut to_merge = Vec::new();
    let to_merge_mutex: Mutex<&mut Vec<(usize, usize)>> = Mutex::new(&mut to_merge);

    population
        .par_iter_mut()
        .zip(0..POP_SIZE)
        .for_each(|(computed_particle_a, particle_a_index)| {
            let particle_a = &previous_population[particle_a_index];

            if particle_a.mass != 0f64 {
                for (particle_b_index, particle_b) in previous_population.iter().enumerate() {
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
                        let direction = particle_b.position.0[i] - particle_a.position.0[i];
                        computed_particle_a.speed.0[i] += direction * force_by_mass_a;
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
                computed_particle_a.position.0[i] += particle_a.speed.0[i];
            }
        });

    for (particle_a_index, particle_b_index) in to_merge.iter() {
        let particle_a = population[*particle_a_index];
        let particle_b = population[*particle_b_index];
        if particle_a.mass == 0f64 || particle_b.mass == 0f64 {
            continue;
        }
        let (index_to_fuse, index_to_delete) = if particle_a.mass > particle_b.mass {
            (*particle_a_index, *particle_b_index)
        } else {
            (*particle_b_index, *particle_a_index)
        };
        population[index_to_delete].mass = 0f64;
        population[index_to_fuse].mass = particle_a.mass + particle_b.mass;
        population[index_to_fuse].position.0 = array::from_fn(|i| {
            (particle_a.position.0[i] * particle_a.mass + particle_b.position.0[i] * particle_b.mass)
                / (particle_a.mass + particle_b.mass)
        });
        population[index_to_fuse].speed.0 = array::from_fn(|i| {
            (particle_a.speed.0[i] * particle_a.mass + particle_b.speed.0[i] * particle_b.mass)
                / (particle_a.mass + particle_b.mass)
        });
    }
}

#[cfg(test)]
pub mod test {
    use crate::physics::{Particle, Population, apply_force, Coordinates};

    #[test]
    fn test_apply_force() {
        let mut population: Population = [
            Particle {
                mass: 3f64,
                speed: Coordinates([0f64, 0f64]),
                position: Coordinates([10f64, 10f64]),
            },
            Particle {
                mass: 2f64,
                speed: Coordinates([0f64, 0f64]),
                position: Coordinates([-10f64, -10f64]),
            },
            Particle {
                mass: 1f64,
                speed: Coordinates([0f64, 0f64]),
                position: Coordinates([10f64, -10f64]),
            },
        ];
        for _ in 0..100 {
            population = apply_force(&population);
        }
        assert_eq!(population[0].position, [-2.6124097114690477, -41.87865599101741])
    }
}
