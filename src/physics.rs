// Responsible for defining newtonian physic

use rand::Rng;

pub const DIMENSIONS: usize = 2;
type Coordinates = [f64; DIMENSIONS];

const DEFAULT_COORDINATES: Coordinates = [0f64; DIMENSIONS];

pub const POP_SIZE: usize = 100;

const G: f64 = 50f64;

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

    // pub fn new_test_pop() -> Population {
    //     return [
    //         Particle {
    //             mass: 10f64,
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
    //         Particle {
    //             mass: 10f64,
    //             id: 3,
    //             speed: DEFAULT_COORDINATES,
    //             position: [-100f64, 100f64],
    //         },
    //     ];
    // }
}

pub fn distance(a: Coordinates, b: Coordinates) -> f64 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }
    sum.sqrt()
}

pub fn gravity(affecting_particle: &Particle, affected_particle: &Particle, distance: f64) -> f64 {
    return G * (affected_particle.mass * affecting_particle.mass) / distance.powf(2f64);
}

pub fn apply_force(
    particles: &[Particle; POP_SIZE],
    force_generators: Vec<fn(&Particle, &Particle, f64) -> f64>,
) -> Population {
    let mut computed_particles = DEFAULT_POP;
    let mut affected_particle_index = 0;
    for affected_particle in particles {
        let mut acc: Coordinates = DEFAULT_COORDINATES;
        for affecting_particle in particles {
            if affected_particle.id == affecting_particle.id {
                continue;
            }
            let distance =
                distance(affected_particle.position, affecting_particle.position);
            let mut force = 0f64;
            for force_generator in &force_generators {
                force += force_generator(affecting_particle, affected_particle, distance);
            }
            for i in 0..DIMENSIONS {
                acc[i] += force
                    * ((affecting_particle.position[i] - affected_particle.position[i]) / distance)
                    / affected_particle.mass;
            }
        }
        let mut new_speed = DEFAULT_COORDINATES;
        let mut new_position = DEFAULT_COORDINATES;
        for i in 0..DIMENSIONS {
            new_position[i] = affected_particle.position[i] + affected_particle.speed[i];
            new_speed[i] = affected_particle.speed[i] + acc[i];
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
