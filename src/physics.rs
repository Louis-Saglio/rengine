// Responsible for defining newtonian physic

use rand::Rng;

pub const DIMENSIONS: usize = 3;
type Coordinates = [f64; DIMENSIONS];

const DEFAULT_COORDINATES: Coordinates = [0f64; DIMENSIONS];

pub const POP_SIZE: usize = 1000;

const G: f64 = 0.1;

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
