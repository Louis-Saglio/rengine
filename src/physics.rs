// Responsible for defining newtonian physic

use rand::Rng;

use load_env_var_as_usize::{
    get_dimensions_from_env_var, get_g_from_env_var, get_minimal_distance_from_env_var, get_pop_size_from_env_var,
};

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

const NBR_OF_POSSIBLE_PARTICLE_PAIRS: usize = (POP_SIZE as f64 * ((POP_SIZE - 1) as f64 / 2f64)) as usize;

const fn compute_possible_particle_pairs() -> [(usize, usize); NBR_OF_POSSIBLE_PARTICLE_PAIRS] {
    let mut combinations = [(0, 0); NBR_OF_POSSIBLE_PARTICLE_PAIRS];
    let mut i = 0;
    let mut n = 0;
    loop {
        if i == POP_SIZE {
            break;
        }
        let mut j = i + 1;
        loop {
            if j == POP_SIZE {
                break;
            }
            combinations[n] = (i, j);
            n += 1;
            j += 1;
        }
        i += 1;
    }
    return combinations;
}

pub const POSSIBLE_PARTICLE_PAIRS: [(usize, usize); NBR_OF_POSSIBLE_PARTICLE_PAIRS] = compute_possible_particle_pairs();

#[derive(Clone, Copy)]
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
        return pop;
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
                position[i] = rng.gen();
            }
            pop[i] = Self {
                mass: rng.gen_range(10f64..100f64),
                speed: DEFAULT_COORDINATES,
                position,
            };
        }
        return pop;
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
        return pop;
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

fn distance(a: Coordinates, b: Coordinates) -> f64 {
    return distance_squared(a, b).sqrt();
}

pub fn compute_acceleration_for_particle_pairs(
    particles: &Population,
    pairs: &[Option<(usize, usize)>],
) -> [Coordinates; POP_SIZE] {
    let mut acceleration = [[0f64; DIMENSIONS]; POP_SIZE];
    for optional_pair in pairs {
        match optional_pair {
            None => {}
            Some((particle_a_index, particle_b_index)) => {
                let particle_a = &particles[*particle_a_index];
                let particle_b = &particles[*particle_b_index];
                if particle_a.mass == 0f64 || particle_b.mass == 0f64 {
                    continue;
                }
                let distance = distance(particle_a.position, particle_b.position);
                let g_by_d_cubed = G / (distance * distance * distance);
                let force_by_mass_a_by_distance = particle_b.mass * g_by_d_cubed;
                let force_by_mass_b_by_distance = particle_a.mass * g_by_d_cubed;
                for i in 0..DIMENSIONS {
                    let direction = particle_b.position[i] - particle_a.position[i];
                    acceleration[*particle_a_index][i] += direction * force_by_mass_a_by_distance;
                    acceleration[*particle_b_index][i] -= direction * force_by_mass_b_by_distance;
                }
            }
        }
    }
    return acceleration;
}

pub fn apply_force_by_iterating_over_possible_particle_pairs(particles: &Population) -> Population {
    let mut computed_particles = particles.clone();
    for i in 0..POP_SIZE {
        for j in 0..DIMENSIONS {
            computed_particles[i].position[j] += computed_particles[i].speed[j];
        }
    }
    for (particle_a_index, particle_b_index) in POSSIBLE_PARTICLE_PAIRS {
        let particle_a = &particles[particle_a_index];
        let particle_b = &particles[particle_b_index];
        if particle_a.mass == 0f64 || particle_b.mass == 0f64 {
            continue;
        }
        let distance = distance(particle_a.position, particle_b.position);
        let g_by_d_cubed = G / (distance * distance * distance);
        let force_by_mass_a_by_distance = particle_b.mass * g_by_d_cubed;
        let force_by_mass_b_by_distance = particle_a.mass * g_by_d_cubed;
        for i in 0..DIMENSIONS {
            let direction = particle_b.position[i] - particle_a.position[i];
            computed_particles[particle_a_index].speed[i] += direction * force_by_mass_a_by_distance;
            computed_particles[particle_b_index].speed[i] -= direction * force_by_mass_b_by_distance;
        }
    }
    return computed_particles;
}

pub fn apply_force(particles: &Population) -> Population {
    let mut computed_particles = particles.clone();
    let mut to_merge: Vec<(usize, usize)> = Vec::new();
    for particle_a_index in 0..POP_SIZE {
        let particle_a = &particles[particle_a_index];
        if particle_a.mass == 0f64 {
            continue;
        }
        for particle_b_index in particle_a_index + 1..POP_SIZE {
            let particle_b = &particles[particle_b_index];
            if particle_b.mass == 0f64 {
                continue;
            }
            let distance_squared = distance_squared(particle_a.position, particle_b.position);
            let g_by_d_squared = G / (distance_squared);
            let inverse_distance_square_root = 1f64 / distance_squared.sqrt();
            let force_by_mass_a = particle_b.mass * g_by_d_squared * inverse_distance_square_root;
            let force_by_mass_b = particle_a.mass * g_by_d_squared * inverse_distance_square_root;
            for i in 0..DIMENSIONS {
                let direction = particle_b.position[i] - particle_a.position[i];
                computed_particles[particle_a_index].speed[i] += direction * force_by_mass_a;
                computed_particles[particle_b_index].speed[i] -= direction * force_by_mass_b;
            }
            if distance_squared < MINIMAL_DISTANCE_SQUARED {
                to_merge.push((particle_a_index, particle_b_index));
            }
        }
        for i in 0..DIMENSIONS {
            computed_particles[particle_a_index].position[i] += particle_a.speed[i];
        }
    }
    for (particle_a_index, particle_b_index) in to_merge {
        let particle_a = computed_particles[particle_a_index];
        let particle_b = computed_particles[particle_b_index];
        if particle_a.mass == 0f64 || particle_b.mass == 0f64 {
            continue;
        }
        computed_particles[particle_a_index].mass = 0f64;
        computed_particles[particle_b_index].mass += particle_a.mass;
        for i in 0..DIMENSIONS {
            computed_particles[particle_b_index].position[i] = (particle_a.position[i] * particle_a.mass
                + particle_b.position[i] * particle_b.mass)
                / (particle_a.mass + particle_b.mass);
            computed_particles[particle_b_index].speed[i] = (particle_a.speed[i] * particle_a.mass
                + particle_b.speed[i] * particle_b.mass)
                / (particle_a.mass + particle_b.mass)
        }
    }
    return computed_particles;
}

pub mod distributed {
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{Arc, Mutex};

    use crate::physics::{
        distance_squared, Coordinates, Population, DEFAULT_COORDINATES, DIMENSIONS, G, NBR_OF_POSSIBLE_PARTICLE_PAIRS,
        POP_SIZE,
    };

    /// This is the code that will run into a worker.
    /// It:
    /// 1. reads a pair of particles index from receiver_from_main_thread
    /// 2. computes the acceleration to apply to these particles based on the force they create on each other
    /// 3. send back this information to the main thread through sender_to_main_thread
    pub fn compute_acceleration_in_worker(
        particles: &Population,
        receiver_from_main_thread: Arc<Mutex<Receiver<(usize, usize)>>>,
        sender_to_main_thread: Sender<((usize, Coordinates), (usize, Coordinates))>,
    ) {
        loop {
            let (particle_a_index, particle_b_index) = receiver_from_main_thread.lock().unwrap().recv().unwrap();
            let particle_a = &particles[particle_a_index];
            let particle_b = &particles[particle_b_index];
            let mut acceleration_a = DEFAULT_COORDINATES;
            let mut acceleration_b = DEFAULT_COORDINATES;
            let distance_squared = distance_squared(particle_a.position, particle_b.position);
            let g_by_d_squared = G / (distance_squared);
            let inverse_distance_square_root = 1f64 / distance_squared.sqrt();
            let force_by_mass_a = particle_b.mass * g_by_d_squared * inverse_distance_square_root;
            let force_by_mass_b = particle_a.mass * g_by_d_squared * inverse_distance_square_root;
            for i in 0..DIMENSIONS {
                let direction = particle_b.position[i] - particle_a.position[i];
                acceleration_a[i] += direction * force_by_mass_a;
                acceleration_b[i] -= direction * force_by_mass_b;
            }
            let _ =
                sender_to_main_thread.send(((particle_a_index, acceleration_a), (particle_b_index, acceleration_b)));
        }
    }

    pub fn apply_force_with_workers(
        particles: &Population,
        sender_to_workers: &Sender<(usize, usize)>,
        receiver_from_workers: &Receiver<((usize, Coordinates), (usize, Coordinates))>,
    ) -> Population {
        let mut computed_particles = particles.clone(); // Bug here: the population that the workers have is no longer valid
        for particle_a_index in 0..POP_SIZE {
            let particle_a = &particles[particle_a_index];
            if particles[particle_a_index].mass == 0f64 {
                continue;
            }
            for particle_b_index in particle_a_index + 1..POP_SIZE {
                if particles[particle_b_index].mass == 0f64 {
                    continue;
                }
                let result = sender_to_workers.send((particle_a_index, particle_b_index));
                if result.is_err() {
                    println!("Failed to send {:?} to workers", (particle_a_index, particle_b_index));
                }
            }
            for i in 0..DIMENSIONS {
                computed_particles[particle_a_index].position[i] += particle_a.speed[i];
            }
        }
        for _ in 0..NBR_OF_POSSIBLE_PARTICLE_PAIRS {
            let ((particle_a_index, particle_a_acc), (particle_b_index, particle_b_acc)) =
                receiver_from_workers.recv().unwrap();
            for i in 0..DIMENSIONS {
                computed_particles[particle_a_index].speed[i] += particle_a_acc[i];
                computed_particles[particle_b_index].speed[i] += particle_b_acc[i];
            }
        }
        return computed_particles;
    }
}
