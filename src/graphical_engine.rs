use std::collections::HashMap;

use piston_window::{clear, ellipse, Context, Event, G2d, Input, Loop, Motion, PistonWindow, WindowSettings, EventLoop, text, Transformed};
use rand::{thread_rng, Rng};

use crate::physics::{apply_force, gravity, Particle, Population};

struct GraphicalCoordinatesCalculator {
    window_size: [u32; 2],
    zoom_level: f64,
    must_clear_screen: bool,
    color_by_particle_id: HashMap<u64, [f64; 3]>,
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

    fn zoom(&mut self, scroll: f64) {
        let new_zoom_level = self.zoom_level + scroll;
        if new_zoom_level > 0f64 {
            self.zoom_level = new_zoom_level;
        }
        self.must_clear_screen = true;
    }

    fn get_particle_color(&mut self, particle_id: u64) -> [f64; 3] {
        if !self.color_by_particle_id.contains_key(&particle_id) {
            let mut rng = thread_rng();
            self.color_by_particle_id
                .insert(particle_id, [rng.gen(), rng.gen(), rng.gen()]);
        };
        return self.color_by_particle_id[&particle_id];
    }
}

struct Engine {
    particles: Population,
    graphical_coordinates_calculator: GraphicalCoordinatesCalculator,
}

impl Engine {
    fn render(&mut self, context: Context, graphics: &mut G2d) {
        if self.graphical_coordinates_calculator.must_clear_screen {
            clear([0.0, 0.0, 0.0, 1.0], graphics);
            // self.graphical_coordinates_calculator.must_clear_screen = false;
        }
        for particle in &self.particles {
            let graphical_coordinates = self
                .graphical_coordinates_calculator
                .compute_graphical_coordinates(&particle);
            let color = self
                .graphical_coordinates_calculator
                .get_particle_color(particle.id);
            ellipse(
                [color[0] as f32, color[1] as f32, color[2] as f32, 1.0],
                graphical_coordinates,
                context.transform,
                graphics,
            )
        }
    }

    fn update(&mut self) {
        self.particles = apply_force(&self.particles, vec![gravity])
    }
}

pub fn run() {
    let width = 1900;
    let height = 1000;
    let mut window: PistonWindow = WindowSettings::new("Rengine", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_max_fps(120);
    window.set_ups(120);

    let mut glyphs = window.load_font("/usr/share/fonts/truetype/malayalam/Suruma.ttf").unwrap();

    let mut engine = Engine {
        // particles: Particle::new_test_pop(),
        particles: Particle::new_random_pop_in_screen(width, height),
        graphical_coordinates_calculator: GraphicalCoordinatesCalculator {
            window_size: [width, height],
            zoom_level: 1f64,
            must_clear_screen: true,
            color_by_particle_id: HashMap::new(),
        },
    };

    while let Some(event) = window.next() {
        match event {
            Event::Input(Input::Move(Motion::MouseScroll(scroll)), _) => {
                engine.graphical_coordinates_calculator.zoom(scroll[1])
            }
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&event, |context, graphics, _device| {
                    engine.render(context, graphics);
                    text::Text::new_color([1.0, 0.0, 0.0, 1.0], 32).draw(
                        "I love you",
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(10.0, 100.0),
                        graphics,
                    ).unwrap();
                    glyphs.factory.encoder.flush(_device);
                });
            }
            Event::Loop(Loop::Update(_)) => engine.update(),
            _ => {}
        }
    }
}
