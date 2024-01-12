// Responsible for defining newtonian physic

use rand::Rng;

pub const DIMENSIONS: usize = 3;
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

fn merge(affecting_particle: &Particle, affected_particle: &Particle) -> Particle {
    let affected_must_be_destroyed = affecting_particle.mass > affected_particle.mass
        || affecting_particle.id > affected_particle.id;
    return if affected_must_be_destroyed {
        Particle {
            id: affected_particle.id,
            mass: 0f64,
            position: DEFAULT_COORDINATES,
            speed: DEFAULT_COORDINATES,
        }
    } else {
        let mass = affecting_particle.mass + affected_particle.mass;
        let mut speed = DEFAULT_COORDINATES;
        for i in 0..DIMENSIONS {
            speed[i] = (affecting_particle.speed[i] * affecting_particle.mass
                + affected_particle.speed[i] * affected_particle.mass)
                / mass;
        }
        Particle {
            id: affected_particle.id,
            mass,
            position: affected_particle.position,
            speed,
        }
    };
}

pub fn apply_force(
    particles: &[Particle; POP_SIZE]
) -> Population {
    let mut computed_particles = DEFAULT_POP;
    let mut affected_particle_index = 0;
    for affected_particle in particles {
        let mut acceleration: Coordinates = DEFAULT_COORDINATES;
        for affecting_particle in particles {
            if affected_particle.id == affecting_particle.id {
                continue;
            }
            let distance = distance(affected_particle.position, affecting_particle.position);
            let force_by_mass = G * affecting_particle.mass / (distance * distance);
            for i in 0..DIMENSIONS {
                acceleration[i] += force_by_mass
                    * ((affecting_particle.position[i] - affected_particle.position[i]) / distance);
            }
        }
        let mut new_speed = DEFAULT_COORDINATES;
        let mut new_position = DEFAULT_COORDINATES;
        for i in 0..DIMENSIONS {
            new_position[i] = affected_particle.position[i] + affected_particle.speed[i];
            new_speed[i] = affected_particle.speed[i] + acceleration[i];
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
