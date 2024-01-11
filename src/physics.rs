// Responsible for defining newtonian physic

use rand::Rng;

pub const DIMENSIONS: usize = 2;
type Coordinates = [f64; DIMENSIONS];

const DEFAULT_COORDINATES: Coordinates = [0f64; DIMENSIONS];

pub const POP_SIZE: usize = 4;

const G: f64 = 0.9;

const DEFAULT_PARTICLE: Particle = Particle {
    mass: 0f64,
    speed: [0f64; DIMENSIONS],
    position: [0f64; DIMENSIONS],
    id: 0,
};

pub type Population = [Particle; POP_SIZE];

const DEFAULT_POP: Population = [DEFAULT_PARTICLE; POP_SIZE];

pub struct Particle {
    pub mass: f64,
    pub speed: Coordinates,
    pub position: Coordinates,
    pub id: u64,
}

impl Particle {
    fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            mass: 1f64,
            speed: [0f64; DIMENSIONS],
            position: rng.gen(),
            id: rng.gen(),
        }
    }

    pub fn new_random_pop() -> Population {
        let mut pop = DEFAULT_POP;
        for i in 0..POP_SIZE {
            pop[i] = Self::new_random();
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
                mass: 1f64,
                speed: [0f64; DIMENSIONS],
                position,
                id: rng.gen(),
            };
        }
        return pop;
    }
}

fn distance(a: Coordinates, b: Coordinates) -> f64 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }
    sum.sqrt()
}

pub fn apply_force(particles: &[Particle; POP_SIZE]) -> Population {
    let mut computed_particles = DEFAULT_POP;
    let mut affected_particle_index = 0;
    for affected_particle in particles {
        let mut acc: Coordinates = DEFAULT_COORDINATES;
        for affecting_particle in particles {
            if affected_particle.id == affecting_particle.id {
                continue;
            }
            let force = G * (affected_particle.mass * affecting_particle.mass)
                / distance(affected_particle.position, affecting_particle.position).powf(2f64);
            for i in 0..DIMENSIONS {
                acc[i] = force * (affecting_particle.position[i] - affected_particle.position[i])
                    / affected_particle.mass;
            }
        }
        let mut new_speed = DEFAULT_COORDINATES;
        let mut new_position = DEFAULT_COORDINATES;
        for i in 0..DIMENSIONS {
            new_speed[i] = affected_particle.speed[i] + acc[i];
            new_position[i] = affected_particle.position[i] + new_speed[i];
        }
        computed_particles[affected_particle_index] = Particle {
            id: affected_particle.id,
            mass: affected_particle.mass,
            speed: new_speed,
            position: new_position,
        };
        affected_particle_index += 1;
    }
    return computed_particles;
}
