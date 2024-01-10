use std::collections::HashMap;
use piston_window::{
    clear, ellipse, Context, Event, G2d, Input, Loop, Motion, PistonWindow, WindowSettings,
};
use rand::{Rng, thread_rng};
use rengine::{apply_force, Particle, POP_SIZE};

struct GraphicalCoordinatesCalculator {
    window_size: [u32; 2],
    zoom_level: f64,
    must_clear_screen: bool,
    color_by_particle_id: HashMap<u64, [f64; 3]>
}

impl GraphicalCoordinatesCalculator {
    fn compute_graphical_coordinates(&self, particle: &Particle) -> [f64; 4] {
        let size = [5f64, 5f64];
        let mut coordinates = [particle.position[0], particle.position[1]];
        // Zoom
        coordinates[0] *= self.zoom_level;
        coordinates[1] *= self.zoom_level;
        // Move point 0,0 in the middle of the screen
        coordinates[0] += (self.window_size[0] / 2) as f64;
        coordinates[1] += (self.window_size[1] / 2) as f64;
        // The graphical coordinates is the top left corner of the box around the ellipse
        coordinates[0] -= size[0] / 2f64;
        coordinates[1] -= size[1] / 2f64;
        return [coordinates[0], coordinates[1], size[0], size[1]];
    }

    fn zoom(&mut self, scroll: [f64; 2]) {
        let new_zoom_level = self.zoom_level + scroll[1];
        if new_zoom_level > 0f64 {
            self.zoom_level = new_zoom_level;
        }
        self.must_clear_screen = true;
    }

    fn get_particle_color(&mut self, particle_id: u64) -> [f64; 3] {
        if !self.color_by_particle_id.contains_key(&particle_id) {
            let mut rng = thread_rng();
            self.color_by_particle_id.insert(particle_id, [rng.gen(), rng.gen(), rng.gen()]);
        };
        return self.color_by_particle_id[&particle_id]
    }
}

struct Engine {
    particles: [Particle; POP_SIZE],
    graphical_coordinates_calculator: GraphicalCoordinatesCalculator,
}

impl Engine {
    fn render(&mut self, context: Context, graphics: &mut G2d) {
        if self.graphical_coordinates_calculator.must_clear_screen {
            clear([0.0, 0.0, 0.0, 1.0], graphics);
            self.graphical_coordinates_calculator.must_clear_screen = false;
        }
        for particle in &self.particles {
            let graphical_coordinates = self
                .graphical_coordinates_calculator
                .compute_graphical_coordinates(&particle);
            let color = self.graphical_coordinates_calculator.get_particle_color(particle.id);
            ellipse(
                [color[0] as f32, color[1] as f32, color[2] as f32, 1.0],
                graphical_coordinates,
                context.transform,
                graphics,
            )
        }
    }

    fn update(&mut self) {
        self.particles = apply_force(&self.particles)
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rengine", [1000, 800])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut engine = Engine {
        particles: Particle::new_random_pop(),
        graphical_coordinates_calculator: GraphicalCoordinatesCalculator {
            window_size: [1000, 800],
            zoom_level: 5f64,
            must_clear_screen: true,
            color_by_particle_id: HashMap::new()
        },
    };

    while let Some(event) = window.next() {
        match event {
            Event::Input(Input::Move(Motion::MouseScroll(scroll)), _) => {
                engine.graphical_coordinates_calculator.zoom(scroll)
            }
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&event, |context, graphics, _device| {
                    engine.render(context, graphics);
                });
            }
            Event::Loop(Loop::Update(_)) => engine.update(),
            _ => {}
        }
    }
}
