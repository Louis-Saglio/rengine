use rand::Rng;

const DIMENSIONS: usize = 2;
type Coordinates = [f64; DIMENSIONS];

pub const POP_SIZE: usize = 100;

const G: f64 = 0.1;

const DEFAULT_PARTICLE: Particle = Particle {
    mass: 1f64,
    speed: [0f64; DIMENSIONS],
    position: [0f64; DIMENSIONS],
    id: 1,
};

const DEFAULT_POP: [Particle; POP_SIZE] = [DEFAULT_PARTICLE; POP_SIZE];

pub struct Particle {
    mass: f64,
    speed: Coordinates,
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

    pub fn new_random_pop() -> [Particle; POP_SIZE] {
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

pub fn apply_force(particles: &[Particle; POP_SIZE]) -> [Particle; POP_SIZE] {
    let mut computed_particles = DEFAULT_POP;
    let mut i = 0;
    for particle_a in particles {
        let mut acc: Coordinates = [0f64; DIMENSIONS];
        for particle_b in particles {
            if particle_a.id == particle_b.id {
                continue;
            }
            let force = G * (particle_a.mass * particle_b.mass)
                / distance(particle_a.position, particle_b.position).powf(2f64);
            for i in 0..DIMENSIONS {
                acc[i] =
                    force * (particle_b.position[i] - particle_a.position[i]) / particle_a.mass;
            }
        }
        let mut new_speed = [0f64; DIMENSIONS];
        let mut new_position = [0f64; DIMENSIONS];
        for i in 0..DIMENSIONS {
            new_speed[i] = particle_a.speed[i] + acc[i];
            new_position[i] = particle_a.position[i] + new_speed[i];
        }
        computed_particles[i] = Particle {
            id: particle_a.id,
            mass: particle_a.mass,
            speed: new_speed,
            position: new_position,
        };
        i += 1;
    }
    return computed_particles;
}
