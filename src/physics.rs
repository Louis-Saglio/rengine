// Responsible for defining newtonian physic

use std::thread;
use rand::Rng;

pub const DIMENSIONS: usize = 2;
type Coordinates = [f64; DIMENSIONS];

const DEFAULT_COORDINATES: Coordinates = [0f64; DIMENSIONS];

pub const POP_SIZE: usize = 1000;

const G: f64 = 0.005f64;
const MINIMAL_DISTANCE: f64 = 3f64;

const DEFAULT_PARTICLE: Particle = Particle {
    mass: 0f64,
    speed: DEFAULT_COORDINATES,
    position: DEFAULT_COORDINATES,
};

pub type Population = [Particle; POP_SIZE];

const DEFAULT_POP: Population = [DEFAULT_PARTICLE; POP_SIZE];

const NBR_OF_POSSIBLE_PARTICLE_PAIRS: usize = (POP_SIZE as f64 * ((POP_SIZE - 1) as f64 / 2f64)) as usize;

const WORKER_NBR: usize = 2;

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

const CHUNK_SIZE: usize = (NBR_OF_POSSIBLE_PARTICLE_PAIRS / WORKER_NBR) + 1;

type Chunk = [Option<(usize, usize)>; CHUNK_SIZE];

type Chunks = [Chunk; WORKER_NBR];

const fn compute_chunks() -> Chunks {
    let mut chunks = [[None; CHUNK_SIZE]; WORKER_NBR];
    let mut chunk_index = 0;
    let mut chunk_item_index = 0;
    let mut pair_index = 0;
    let mut nbr_of_pair_in_chunk = 0;
    loop {
        if pair_index == NBR_OF_POSSIBLE_PARTICLE_PAIRS { break }
        if nbr_of_pair_in_chunk == CHUNK_SIZE {
            chunk_index += 1;
            chunk_item_index = 0;
        }
        chunks[chunk_index][chunk_item_index] = Some(POSSIBLE_PARTICLE_PAIRS[pair_index]);
        nbr_of_pair_in_chunk += 1;
        chunk_item_index += 1;
        pair_index += 1;
    }
    return chunks;
}

const POSSIBLE_PARTICLE_PAIRS_CHUNKS: [[Option<(usize, usize)>; CHUNK_SIZE]; WORKER_NBR] = compute_chunks();

type AccelerationBucket = [Coordinates; POP_SIZE];
type AccelerationBuckets = [AccelerationBucket; WORKER_NBR];

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

    // pub fn new_test_pop() -> Population {
    //     return [
    //         Particle {
    //             mass: 15f64,
    //             speed: DEFAULT_COORDINATES,
    //             position: [100f64, 100f64],
    //         },
    //         Particle {
    //             mass: 10f64,
    //             speed: DEFAULT_COORDINATES,
    //             position: [100f64, -100f64],
    //         },
    //         Particle {
    //             mass: 10f64,
    //             speed: DEFAULT_COORDINATES,
    //             position: [-100f64, -100f64],
    //         },
    //         // Particle {
    //         //     mass: 10f64,
    //         //     id: 3,
    //         //     speed: DEFAULT_COORDINATES,
    //         //     position: [-100f64, 100f64],
    //         // },
    //     ];
    // }
}

pub fn distance(a: Coordinates, b: Coordinates) -> f64 {
    let mut sum = 0.0;
    for i in 0..DIMENSIONS {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }
    sum.sqrt()
}

pub fn compute_acceleration_for_particle_pairs(
    particles: &Population,
    pairs: &[Option<(usize, usize)>],
) -> [Coordinates; POP_SIZE] {
    let mut acceleration = [[0f64; DIMENSIONS]; POP_SIZE];
    for optional_pair in pairs {
        match optional_pair {
            None => {},
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

pub fn accelerate_particles_from_acceleration_buckets(
    particles: &Population,
    acceleration_buckets: AccelerationBuckets,
) -> Population {
    let mut computed_particles = particles.clone();
    for particle_index in 0..POP_SIZE {
        for i in 0..DIMENSIONS {
            computed_particles[particle_index].position[i] += computed_particles[particle_index].speed[i]
        }
    }
    for bucket in acceleration_buckets {
        for (particle_index, acceleration) in bucket.iter().enumerate() {
            for i in 0..DIMENSIONS {
                computed_particles[particle_index].speed[i] += acceleration[i];
            }
        }
    }
    return computed_particles;
}

pub fn apply_force_multi_threaded(particles: &Population) -> Population {
    let particles= particles.clone();
    let mut acceleration_buckets: AccelerationBuckets = [[DEFAULT_COORDINATES; POP_SIZE]; WORKER_NBR];
    let threads = (0..WORKER_NBR).map(|i| {
        thread::spawn(move || {
            compute_acceleration_for_particle_pairs(&particles.clone(), &POSSIBLE_PARTICLE_PAIRS_CHUNKS[i])
        })
    });
    for (i, thread) in threads.into_iter().enumerate() {
        match thread.join() {
            Ok(acceleration_bucket) => {
                acceleration_buckets[i] = acceleration_bucket;
            }
            Err(_) => {}
        }
    }
    return accelerate_particles_from_acceleration_buckets(&particles, acceleration_buckets);
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
            let distance = distance(particle_a.position, particle_b.position);
            let g_by_d_cubed = G / (distance * distance * distance);
            let force_by_mass_a_by_distance = particle_b.mass * g_by_d_cubed;
            let force_by_mass_b_by_distance = particle_a.mass * g_by_d_cubed;
            for i in 0..DIMENSIONS {
                let direction = particle_b.position[i] - particle_a.position[i];
                computed_particles[particle_a_index].speed[i] += direction * force_by_mass_a_by_distance;
                computed_particles[particle_b_index].speed[i] -= direction * force_by_mass_b_by_distance;
            }
            if distance < MINIMAL_DISTANCE {
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
