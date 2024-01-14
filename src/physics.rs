// Responsible for defining newtonian physic

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
    //             id: 0,
    //             speed: DEFAULT_COORDINATES,
    //             position: [100f64, 100f64],
    //         },
    //         Particle {
    //             mass: 10f64,
    //             id: 1,
    //             speed: DEFAULT_COORDINATES,
    //             position: [100f64, -100f64],
    //         },
    //         Particle {
    //             mass: 10f64,
    //             id: 2,
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

pub fn apply_force(particles: &[Particle; POP_SIZE]) -> Population {
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
                computed_particles[particle_a_index].speed[i] +=
                    direction * force_by_mass_a_by_distance;
                computed_particles[particle_b_index].speed[i] -=
                    direction * force_by_mass_b_by_distance;
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
            computed_particles[particle_b_index].position[i] = (particle_a.position[i]
                * particle_a.mass
                + particle_b.position[i] * particle_b.mass)
                / (particle_a.mass + particle_b.mass);
            computed_particles[particle_b_index].speed[i] = (particle_a.speed[i] * particle_a.mass
                + particle_b.speed[i] * particle_b.mass)
                / (particle_a.mass + particle_b.mass)
        }
    }
    return computed_particles;
}
