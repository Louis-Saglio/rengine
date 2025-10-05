use crate::build_variant::{BUILD_VARIANT, DEMO_BV, TEST_BV};
use piston_window::{
    clear, ellipse, text, Button, Context, Event, EventLoop, G2d, Input, Key, Loop, Motion, PistonWindow, Transformed,
    WindowSettings,
};
use rand::{rng, Rng};
use std::collections::HashMap;
use std::time::Instant;

use crate::physics::{apply_force, ApplyForceContext, Particle};

struct GraphicalCoordinatesCalculator {
    window_size: [u32; 2],
    zoom_level: f64,
    shift: [f64; 2],
    must_clear_screen: bool,
    color_by_particle_id: HashMap<u64, [f64; 3]>,
}

impl GraphicalCoordinatesCalculator {
    fn compute_graphical_coordinates(&self, particle: &Particle) -> [f64; 4] {
        let size = [particle.mass.sqrt(); 2];
        let mut coordinates = [particle.position[0], particle.position[1]];
        // Zoom
        coordinates[0] *= self.zoom_level;
        coordinates[1] *= self.zoom_level;
        // Move point 0,0 in the middle of the screen
        coordinates[0] += (self.window_size[0] / 2) as f64;
        coordinates[1] += (self.window_size[1] / 2) as f64;
        // Shift the window
        coordinates[0] += self.shift[0] * 10f64;
        coordinates[1] += self.shift[1] * 10f64;
        // The graphical coordinates is the top left corner of the box around the ellipse
        coordinates[0] -= size[0] / 2f64;
        coordinates[1] -= size[1] / 2f64;
        [coordinates[0], coordinates[1], size[0], size[1]]
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
            let mut rng = rng();
            self.color_by_particle_id
                .insert(particle_id, [rng.random(), rng.random(), rng.random()]);
        };
        self.color_by_particle_id[&particle_id]
    }
}

struct Engine {
    context: ApplyForceContext,
    graphical_coordinates_calculator: GraphicalCoordinatesCalculator,
    fps: f64,                 // Added FPS field
    last_frame_time: Instant, // Track the last frame time
    ups: f64,
    last_update_time: Instant,
}

impl Engine {
    fn render(&mut self, context: Context, graphics: &mut G2d) {
        if self.graphical_coordinates_calculator.must_clear_screen {
            clear([0.0, 0.0, 0.0, 1.0], graphics);
            // self.graphical_coordinates_calculator.must_clear_screen = false;
        }
        for (index, particle) in self.context.population.iter().enumerate() {
            if particle.mass == 0f64 {
                continue;
            }
            let graphical_coordinates = self
                .graphical_coordinates_calculator
                .compute_graphical_coordinates(&particle);
            let color = self.graphical_coordinates_calculator.get_particle_color(index as u64);
            ellipse(
                [color[0] as f32, color[1] as f32, color[2] as f32, 1.0],
                graphical_coordinates,
                context.transform,
                graphics,
            )
        }
        self.update_fps();
    }

    fn update(&mut self) {
        apply_force(&mut self.context);
        self.update_ups();
    }

    fn update_fps(&mut self) {
        let now = Instant::now();
        let duration = now.duration_since(self.last_frame_time);
        self.fps = 1.0 / duration.as_secs_f64();
        self.last_frame_time = now;
    }

    fn update_ups(&mut self) {
        let now = Instant::now();
        let duration = now.duration_since(self.last_update_time);
        self.ups = 1.0 / duration.as_secs_f64();
        self.last_update_time = now;
    }
}

pub fn run() {
    let width = 1900;
    let height = 1000;
    let mut window: PistonWindow = WindowSettings::new("Rengine", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_max_fps(600);
    window.set_ups(600);

    let mut glyphs = window
        .load_font("/usr/share/fonts/truetype/freefont/FreeMono.ttf")
        .unwrap();

    let particles = if BUILD_VARIANT == TEST_BV {
        Particle::new_test_pop()
    } else if BUILD_VARIANT == DEMO_BV {
        Particle::new_random_pop_in_screen(width, height)
    } else {
        Particle::new_random_pop()
    };

    let mut engine = Engine {
        context: ApplyForceContext { population: particles },
        graphical_coordinates_calculator: GraphicalCoordinatesCalculator {
            window_size: [width, height],
            zoom_level: 1f64,
            shift: [0f64; 2],
            must_clear_screen: true,
            color_by_particle_id: HashMap::new(),
        },
        fps: 0.0,
        last_frame_time: Instant::now(),
        ups: 0.0,
        last_update_time: Instant::now(),
    };

    while let Some(event) = window.next() {
        match event {
            Event::Input(Input::Move(Motion::MouseScroll(scroll)), _) => {
                engine.graphical_coordinates_calculator.zoom(scroll[1])
            }
            Event::Input(Input::Button(button_args), _) => match button_args.button {
                Button::Keyboard(key) => match key {
                    Key::Right => {
                        engine.graphical_coordinates_calculator.shift[0] += 1f64;
                    }
                    Key::Left => {
                        engine.graphical_coordinates_calculator.shift[0] -= 1f64;
                    }
                    Key::Down => {
                        engine.graphical_coordinates_calculator.shift[1] += 1f64;
                    }
                    Key::Up => {
                        engine.graphical_coordinates_calculator.shift[1] -= 1f64;
                    }
                    _ => {}
                },
                _ => {}
            },
            Event::Loop(Loop::Render(_)) => {
                let nbr = engine
                    .context
                    .population
                    .iter()
                    .filter(|&particle| particle.mass != 0f64)
                    .count();
                window.draw_2d(&event, |context, graphics, _device| {
                    engine.render(context, graphics);
                    text::Text::new_color([1.0, 0.0, 0.0, 1.0], 32)
                        .draw(
                            &*format!("{} - {} - {}", nbr, engine.fps.round(), engine.ups.round()),
                            &mut glyphs,
                            &context.draw_state,
                            context.transform.trans(0.0, 32.0),
                            graphics,
                        )
                        .unwrap();
                    glyphs.factory.encoder.flush(_device);
                });
            }
            Event::Loop(Loop::Update(_)) => engine.update(),
            _ => {}
        }
    }
}
