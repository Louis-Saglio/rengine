use crate::input::{InputEvent, KeyCode, KeyState, RelativeAxis, open_input_event_devices, read_input_events};
use crate::physics::{DIMENSIONS, POP_SIZE, Particle, Population, apply_force};
use memmap2::{MmapMut, MmapOptions};
use proc_macros::{get_desired_ups_from_env_var, get_iterations_from_env_var, get_particle_shape_from_env_var};
use rand::random;
use std::array;
use std::fs::{File, OpenOptions};
use std::os::fd::AsRawFd;
use std::thread::sleep;
use std::time::{Duration, Instant};

const BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH: usize = 2560;
const SCREEN_HEIGHT: usize = 1440;

const FRAMEBUFFER_LENGTH: usize = SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL;

const BLACK: [u8; BYTES_PER_PIXEL] = [0, 0, 0, 0];
const WHITE: [u8; BYTES_PER_PIXEL] = [255, 255, 255, 255];

const DESIRED_UPS: u16 = get_desired_ups_from_env_var!();
const DESIRED_UPDATE_DURATION: Duration = if DESIRED_UPS == 0 {
    Duration::ZERO
} else {
    Duration::from_micros(1000000 / DESIRED_UPS as u64)
};

const ITERATIONS: u32 = get_iterations_from_env_var!();

const PARTICLE_SHAPE: &str = get_particle_shape_from_env_var!();

struct Framebuffer {
    mmap: Box<MmapMut>,
}

impl Framebuffer {
    pub fn new() -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/fb0")
            .expect("Unable to open framebuffer device");
        Self::map(&file)
    }

    fn map(file: &File) -> Self {
        let mmap = unsafe {
            MmapOptions::new()
                .len(FRAMEBUFFER_LENGTH)
                .map_mut(file)
                .expect("Unable to mmap framebuffer")
        };
        Framebuffer { mmap: Box::new(mmap) }
    }

    #[cfg(test)]
    fn from_file(path: &std::path::Path) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .expect("Unable to open test framebuffer");
        Self::map(&file)
    }

    pub fn clear(&mut self) {
        self.mmap.fill(0);
    }

    fn fill_span(&mut self, y: usize, x_start: usize, x_end: usize, color: &[u8; BYTES_PER_PIXEL]) {
        let start = (y * SCREEN_WIDTH + x_start) * BYTES_PER_PIXEL;
        let end = (y * SCREEN_WIDTH + x_end + 1) * BYTES_PER_PIXEL;
        for pixel in self.mmap[start..end].chunks_exact_mut(BYTES_PER_PIXEL) {
            pixel.copy_from_slice(color);
        }
    }

    fn draw_pixel(&mut self, x: isize, y: isize, color: &[u8; BYTES_PER_PIXEL]) {
        if x < 0 || x >= SCREEN_WIDTH as isize || y < 0 || y >= SCREEN_HEIGHT as isize {
            return;
        }
        let index = (y as usize * SCREEN_WIDTH + x as usize) * BYTES_PER_PIXEL;
        self.mmap[index..index + BYTES_PER_PIXEL].copy_from_slice(color);
    }

    pub fn draw_square(&mut self, x: isize, y: isize, width: usize, height: usize, color: &[u8; BYTES_PER_PIXEL]) {
        if width == 0 || height == 0 {
            return;
        }
        let (width, height) = (width as isize, height as isize);
        if x + width <= 0 || y + height <= 0 || x >= SCREEN_WIDTH as isize || y >= SCREEN_HEIGHT as isize {
            return;
        }
        let x_start = x.max(0) as usize;
        let x_end = (x + width - 1).min(SCREEN_WIDTH as isize - 1) as usize;
        let y_start = y.max(0) as usize;
        let y_end = (y + height - 1).min(SCREEN_HEIGHT as isize - 1) as usize;
        for py in y_start..=y_end {
            self.fill_span(py, x_start, x_end, color);
        }
    }

    pub fn draw_circle(&mut self, x: isize, y: isize, radius: usize, color: &[u8; BYTES_PER_PIXEL]) {
        let radius = radius as isize;
        if x + radius < 0 || y + radius < 0 || x - radius >= SCREEN_WIDTH as isize || y - radius >= SCREEN_HEIGHT as isize {
            return;
        }
        let y_start = (y - radius).max(0) as usize;
        let y_end = (y + radius).min(SCREEN_HEIGHT as isize - 1) as usize;
        for py in y_start..=y_end {
            let dy = py as isize - y;
            let half_width = ((radius * radius - dy * dy) as usize).isqrt() as isize;
            let x_start = (x - half_width).max(0);
            let x_end = (x + half_width).min(SCREEN_WIDTH as isize - 1);
            if x_start > x_end {
                continue;
            }
            self.fill_span(py, x_start as usize, x_end as usize, color);
        }
    }

    pub fn draw_cross(&mut self, x: isize, y: isize, color: &[u8; BYTES_PER_PIXEL]) {
        let size = 10;
        for i in -size..=size {
            self.draw_pixel(x + i, y, color);
            self.draw_pixel(x, y + i, color);
        }
    }
}

const KDSETMODE: libc::c_ulong = 0x4B3A;
const KD_GRAPHICS: libc::c_long = 0x01;
const KD_TEXT: libc::c_long = 0x00;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct DrawnParticle {
    x: isize,
    y: isize,
    radius: usize,
}

struct RenderState {
    drawn: [DrawnParticle; POP_SIZE],
    mouse: Option<(isize, isize)>,
    sizes: [usize; POP_SIZE],
    masses: [f64; POP_SIZE],
}

impl RenderState {
    fn new(population: &Population) -> Self {
        let mut state = RenderState {
            drawn: [DrawnParticle::default(); POP_SIZE],
            mouse: None,
            sizes: [0; POP_SIZE],
            masses: [0f64; POP_SIZE],
        };
        for (slot, particle) in population.iter().enumerate() {
            state.masses[slot] = particle.mass;
            state.sizes[slot] = particle_radius(particle.mass);
        }
        state
    }

    fn forget_screen(&mut self) {
        self.drawn = [DrawnParticle::default(); POP_SIZE];
        self.mouse = None;
    }
}

fn particle_radius(mass: f64) -> usize {
    if mass == 0f64 {
        0
    } else {
        mass.powf(1.0 / DIMENSIONS as f64) as usize
    }
}

fn screen_particle(particle: &Particle, zoom: f64, shift: (isize, isize), dim_0: usize, dim_1: usize, radius: usize) -> DrawnParticle {
    DrawnParticle {
        x: (particle.position[dim_0] * zoom + (SCREEN_WIDTH as f64 / 2f64)) as isize + shift.0,
        y: (particle.position[dim_1] * zoom + (SCREEN_HEIGHT as f64 / 2f64)) as isize + shift.1,
        radius,
    }
}

fn draw_shape(framebuffer: &mut Framebuffer, drawn: &DrawnParticle, color: &[u8; BYTES_PER_PIXEL]) {
    if PARTICLE_SHAPE == "square" {
        framebuffer.draw_square(drawn.x, drawn.y, drawn.radius, drawn.radius, color);
    } else {
        framebuffer.draw_circle(drawn.x, drawn.y, drawn.radius, color);
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_frame(
    framebuffer: &mut Framebuffer,
    population: &Population,
    particles_colors: &[[u8; BYTES_PER_PIXEL]; POP_SIZE],
    state: &mut RenderState,
    zoom: f64,
    shift: (isize, isize),
    dim_0: usize,
    dim_1: usize,
    clear_between_frames: bool,
    mouse_position: (isize, isize),
) {
    let mut current = [DrawnParticle::default(); POP_SIZE];
    for (slot, particle) in population.iter().enumerate() {
        if particle.mass == 0f64 {
            state.masses[slot] = 0f64;
            state.sizes[slot] = 0;
            continue;
        }
        if particle.mass != state.masses[slot] {
            state.masses[slot] = particle.mass;
            state.sizes[slot] = particle_radius(particle.mass);
        }
        current[slot] = screen_particle(particle, zoom, shift, dim_0, dim_1, state.sizes[slot]);
    }

    if clear_between_frames {
        for (slot, previous) in state.drawn.iter().enumerate() {
            if previous.radius > 0 && *previous != current[slot] {
                draw_shape(framebuffer, previous, &BLACK);
            }
        }
        if state.mouse != Some(mouse_position) {
            if let Some((x, y)) = state.mouse {
                framebuffer.draw_cross(x, y, &BLACK);
            }
        }
    }

    // Always redraw every live particle: an unchanged one may have been partially
    // erased by a neighbour's erase in the pass above.
    for (slot, particle) in current.iter().enumerate() {
        if particle.radius > 0 {
            draw_shape(framebuffer, particle, &particles_colors[slot]);
        }
        state.drawn[slot] = *particle;
    }

    if clear_between_frames && state.mouse != Some(mouse_position) {
        framebuffer.draw_cross(mouse_position.0, mouse_position.1, &WHITE);
        state.mouse = Some(mouse_position);
    }
}

pub fn run(population: &mut Population) {
    let tty = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .expect("Unable to open tty");
    unsafe {
        libc::ioctl(tty.as_raw_fd(), KDSETMODE, KD_GRAPHICS);
    }

    let mut keyboards = open_input_event_devices("kbd");
    let mut mouses = open_input_event_devices("mouse");

    let mut framebuffer = Framebuffer::new();
    let mut render_state = RenderState::new(population);

    let particles_colors: [[u8; BYTES_PER_PIXEL]; POP_SIZE] = array::from_fn(|_| random());

    let mut zoom: f64 = 1.0;
    let mut shift: (isize, isize) = (0, 0);
    let mut clear_between_frames = true;
    let mut needs_full_clear = true;
    let mut limit_ups = true;
    let mut dim_0: usize = 0;
    let mut dim_1: usize = 1;
    let mut mouse_position: (isize, isize) = (SCREEN_WIDTH as isize / 2, SCREEN_HEIGHT as isize / 2);
    let mut quit = false;

    let mut total_simulation_time = Duration::ZERO;
    let mut total_rendering_time = Duration::ZERO;
    let mut total_clearing_screen_time = Duration::ZERO;
    let mut total_input_handling_time = Duration::ZERO;

    let engine_start_instant = Instant::now();

    let mut i = 0;
    loop {
        i += 1;
        if i == ITERATIONS || quit {
            break;
        }
        let update_start = Instant::now();

        let start = Instant::now();
        for kb_event in read_input_events(&mut keyboards) {
            match kb_event {
                InputEvent::KeyPressed { key, state } if state == KeyState::Pressed || state == KeyState::Autorepeat => {
                    match key {
                        KeyCode::Left => shift.0 += 10,
                        KeyCode::Right => shift.0 -= 10,
                        KeyCode::Up => shift.1 += 10,
                        KeyCode::Down => shift.1 -= 10,
                        KeyCode::T => {
                            clear_between_frames = !clear_between_frames;
                            needs_full_clear = needs_full_clear || clear_between_frames;
                        }
                        KeyCode::U => limit_ups = !limit_ups,
                        KeyCode::Q => quit = true,
                        KeyCode::R => {
                            dim_0 = (dim_0 + 1) % DIMENSIONS;
                            dim_1 = (dim_1 + 1) % DIMENSIONS;
                        }
                        _ => {}
                    }
                }
                _ => {},
            }
        }

        for mouse_event in read_input_events(&mut mouses) {
            match mouse_event {
                InputEvent::Relative { axis, value } => match axis {
                    RelativeAxis::X => {
                        mouse_position.0 = (mouse_position.0 + value as isize).clamp(0, SCREEN_WIDTH as isize);
                    }
                    RelativeAxis::Y => {
                        mouse_position.1 = (mouse_position.1 + value as isize).clamp(0, SCREEN_HEIGHT as isize);
                    }
                    RelativeAxis::Wheel => match value {
                        1 => zoom *= 1.1,
                        -1 => zoom *= 0.9,
                        _ => {}
                    },
                    RelativeAxis::Unknown(_) => {}
                },
                InputEvent::KeyPressed {
                    key: KeyCode::MouseLeft,
                    state: KeyState::Pressed,
                } => {
                    println!("Mouse left click at: ({}, {})", mouse_position.0, mouse_position.1);
                }
                _ => {}
            }
        }
        total_input_handling_time += start.elapsed();

        let start = Instant::now();
        apply_force(population);
        total_simulation_time += start.elapsed();

        if needs_full_clear {
            let start = Instant::now();
            framebuffer.clear();
            render_state.forget_screen();
            total_clearing_screen_time += start.elapsed();
            needs_full_clear = false;
        }

        let start = Instant::now();
        draw_frame(
            &mut framebuffer,
            population,
            &particles_colors,
            &mut render_state,
            zoom,
            shift,
            dim_0,
            dim_1,
            clear_between_frames,
            mouse_position,
        );
        total_rendering_time += start.elapsed();

        let update_duration = update_start.elapsed();
        if update_duration < DESIRED_UPDATE_DURATION && limit_ups {
            sleep(DESIRED_UPDATE_DURATION - update_duration);
        }
    }

    unsafe {
        libc::ioctl(tty.as_raw_fd(), KDSETMODE, KD_TEXT);
    }

    println!("UPS: {}", (i as f64 / engine_start_instant.elapsed().as_secs_f64()) as u32);
    println!("Total time: {}ms", engine_start_instant.elapsed().as_millis());
    println!("Simulation time: {}ms", total_simulation_time.as_millis());
    println!("Rendering time: {}ms", total_rendering_time.as_millis());
    println!("Clearing screen time: {}ms", total_clearing_screen_time.as_millis());
    println!("Input handling time: {}ms", total_input_handling_time.as_millis());
    println!(
        "Unaccounted time: {}ms",
        (engine_start_instant.elapsed()
            - total_simulation_time
            - total_rendering_time
            - total_input_handling_time
            - total_clearing_screen_time)
            .as_millis()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::physics::{Coordinates, Particle};

    fn test_framebuffer(name: &str) -> (Framebuffer, std::path::PathBuf) {
        let path = std::env::temp_dir().join(format!("rengine_{}_{}.fb", name, std::process::id()));
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .expect("Unable to create test framebuffer");
        file.set_len(FRAMEBUFFER_LENGTH as u64).expect("Unable to size test framebuffer");
        (Framebuffer::from_file(&path), path)
    }

    fn reference_frame(
        framebuffer: &mut Framebuffer,
        population: &Population,
        colors: &[[u8; BYTES_PER_PIXEL]; POP_SIZE],
        mouse_position: (isize, isize),
        clear: bool,
    ) {
        if clear {
            framebuffer.clear();
        }
        for (slot, particle) in population.iter().enumerate() {
            if particle.mass == 0f64 {
                continue;
            }
            let drawn = screen_particle(particle, 1.0, (0, 0), 0, 1, particle_radius(particle.mass));
            draw_shape(framebuffer, &drawn, &colors[slot]);
        }
        if clear {
            framebuffer.draw_cross(mouse_position.0, mouse_position.1, &WHITE);
        }
    }

    fn run_scenario(clear_between_frames: bool, name: &str) {
        let (mut delta, delta_path) = test_framebuffer(&format!("delta_{}", name));
        let (mut reference, reference_path) = test_framebuffer(&format!("reference_{}", name));
        delta.clear();
        reference.clear();

        let mut population = [Particle::default(); POP_SIZE];
        population[0] = Particle {
            mass: 10f64,
            speed: Coordinates::default(),
            position: Coordinates::new([0f64, 0f64]),
        };
        population[1] = Particle {
            mass: 10f64,
            speed: Coordinates::default(),
            position: Coordinates::new([3f64, -1f64]),
        };
        population[2] = Particle {
            mass: 10f64,
            speed: Coordinates::default(),
            position: Coordinates::new([-2f64, 3f64]),
        };
        // Stationary and overlapping particle 0's path: its pixels get erased by
        // particle 0's erase, so it must be redrawn even though it never moves.
        population[3] = Particle {
            mass: 10f64,
            speed: Coordinates::default(),
            position: Coordinates::new([1f64, 1f64]),
        };
        // Runs off the left edge, exercising clipped spans.
        population[4] = Particle {
            mass: 10f64,
            speed: Coordinates::default(),
            position: Coordinates::new([-1274f64, 0f64]),
        };

        let colors: [[u8; BYTES_PER_PIXEL]; POP_SIZE] = array::from_fn(|slot| [slot as u8, (slot * 7) as u8, (slot * 13) as u8, 255]);
        let mut state = RenderState::new(&population);
        let mut mouse_position = (SCREEN_WIDTH as isize / 2, SCREEN_HEIGHT as isize / 2);

        for frame in 0..8 {
            population[0].position[0] += 1.7;
            population[0].position[1] += 0.9;
            population[1].position[0] -= 0.6;
            population[1].position[1] += 1.4;
            population[2].position[0] += 0.4;
            population[2].position[1] -= 1.1;
            population[4].position[0] -= 3.0;
            if frame >= 5 {
                population[2].mass = 0f64;
            }
            mouse_position.0 += 3;
            mouse_position.1 -= 2;

            draw_frame(
                &mut delta,
                &population,
                &colors,
                &mut state,
                1.0,
                (0, 0),
                0,
                1,
                clear_between_frames,
                mouse_position,
            );
            reference_frame(&mut reference, &population, &colors, mouse_position, clear_between_frames);

            assert!(
                delta.mmap[..] == reference.mmap[..],
                "delta rendering diverged from a full repaint at frame {} ({})",
                frame,
                name
            );
        }

        drop(delta);
        drop(reference);
        std::fs::remove_file(delta_path).expect("Unable to remove delta framebuffer");
        std::fs::remove_file(reference_path).expect("Unable to remove reference framebuffer");
    }

    #[test]
    fn delta_rendering_matches_full_repaint() {
        run_scenario(true, "clear");
    }

    #[test]
    fn trail_rendering_matches_full_repaint() {
        run_scenario(false, "trails");
    }
}
