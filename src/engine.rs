use crate::input::{InputEvent, KeyCode, KeyState, RelativeAxis, open_input_event_devices, read_input_events};
use crate::physics::{DIMENSIONS, POP_SIZE, Population, apply_force};
use memmap2::{MmapMut, MmapOptions};
use proc_macros::{get_desired_ups_from_env_var, get_iterations_from_env_var, get_particle_shape_from_env_var};
use rand::random;
use std::array;
use std::fs::OpenOptions;
use std::os::fd::AsRawFd;
use std::thread::sleep;
use std::time::{Duration, Instant};

const BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH: usize = 2560;
const SCREEN_HEIGHT: usize = 1440;

const FRAMEBUFFER_LENGTH: usize = SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL;

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
    buffer: Vec<u8>,
}

impl Framebuffer {
    pub fn new() -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/fb0")
            .expect("Unable to open framebuffer device");
        let mmap = unsafe {
            MmapOptions::new()
                .len(FRAMEBUFFER_LENGTH)
                .map_mut(&file)
                .expect("Unable to mmap framebuffer")
        };
        Framebuffer {
            mmap: Box::new(mmap),
            buffer: vec![0; FRAMEBUFFER_LENGTH],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn draw_pixel(&mut self, x: isize, y: isize, color: &[u8; BYTES_PER_PIXEL]) {
        if x < 0 || x >= SCREEN_WIDTH as isize || y < 0 || y >= SCREEN_HEIGHT as isize {
            return;
        }
        let anchor_pixel_index = (y * (SCREEN_WIDTH as isize) + x) * (BYTES_PER_PIXEL as isize);
        if anchor_pixel_index >= 0 && anchor_pixel_index + (BYTES_PER_PIXEL as isize) < (FRAMEBUFFER_LENGTH as isize) {
            let anchor_pixel_index = anchor_pixel_index as usize;
            let pixel_slice = &mut self.buffer[anchor_pixel_index..anchor_pixel_index + BYTES_PER_PIXEL];
            pixel_slice.copy_from_slice(color);
        }
    }

    pub fn draw_square(&mut self, x: isize, y: isize, width: usize, height: usize, color: &[u8; BYTES_PER_PIXEL]) {
        for i in x..(x + (width as isize)) {
            for j in y..(y + (height as isize)) {
                self.draw_pixel(i, j, color);
            }
        }
    }

    pub fn draw_circle(&mut self, x: isize, y: isize, radius: usize, color: &[u8; BYTES_PER_PIXEL]) {
        let rsqr = (radius * radius) as isize;
        for dx in -(radius as isize)..=(radius as isize) {
            for dy in -(radius as isize)..=(radius as isize) {
                if dx * dx + dy * dy <= rsqr {
                    let px = x + dx;
                    let py = y + dy;
                    self.draw_pixel(px, py, color);
                }
            }
        }
    }

    pub fn draw_cross(&mut self, x: isize, y: isize, color: &[u8; BYTES_PER_PIXEL]) {
        let size = 10;
        for i in -size..=size {
            self.draw_pixel(x + i, y, color);
            self.draw_pixel(x, y + i, color);
        }
    }

    pub fn draw(&mut self) {
        self.mmap.copy_from_slice(self.buffer.as_slice());
    }
}

const KDSETMODE: libc::c_ulong = 0x4B3A;
const KD_GRAPHICS: libc::c_long = 0x01;
const KD_TEXT: libc::c_long = 0x00;

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

    let particles_colors: [[u8; BYTES_PER_PIXEL]; POP_SIZE] = array::from_fn(|_| random());

    let mut zoom: f64 = 1.0;
    let mut shift: (isize, isize) = (0, 0);
    let mut clear_between_frames = true;
    let mut limit_ups = true;
    let mut dim_0: usize = 0;
    let mut dim_1: usize = 1;
    let mut mouse_position: (i32, i32) = (SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2);
    let mut quit = false;

    let mut total_simulation_time = Duration::ZERO;
    let mut total_rendering_time = Duration::ZERO;
    let mut total_drawing_time = Duration::ZERO;
    let mut total_clearing_screen_time = Duration::ZERO;
    let mut total_input_handling_time = Duration::ZERO;

    let engine_start_instant = Instant::now();

    framebuffer.clear();
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
                InputEvent::Keyboard { key, state } if state == KeyState::Pressed || state == KeyState::Autorepeat => {
                    match key {
                        KeyCode::Left => shift.0 += 10,
                        KeyCode::Right => shift.0 -= 10,
                        KeyCode::Up => shift.1 += 10,
                        KeyCode::Down => shift.1 -= 10,
                        KeyCode::T => clear_between_frames = !clear_between_frames,
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
            if let InputEvent::Relative { axis, value } = mouse_event {
                match axis {
                    RelativeAxis::X => {
                        mouse_position.0 = (mouse_position.0 + value).clamp(0, SCREEN_WIDTH as i32);
                    }
                    RelativeAxis::Y => {
                        mouse_position.1 = (mouse_position.1 + value).clamp(0, SCREEN_HEIGHT as i32);
                    }
                    RelativeAxis::Wheel => match value {
                        1 => zoom *= 1.1,
                        -1 => zoom *= 0.9,
                        _ => {}
                    },
                    RelativeAxis::Unknown(_) => {}
                }
            }
        }
        total_input_handling_time += start.elapsed();

        let start = Instant::now();
        apply_force(population);
        total_simulation_time += start.elapsed();

        let start = Instant::now();
        if clear_between_frames {
            framebuffer.clear();
        }
        total_clearing_screen_time += start.elapsed();

        let start = Instant::now();
        for (particle, particle_color) in population.iter().zip(particles_colors.iter()) {
            if particle.mass == 0.0 {
                continue;
            }
            let size = particle.mass.powf(1.0 / DIMENSIONS as f64) as usize;
            if PARTICLE_SHAPE == "square" {
                framebuffer.draw_square(
                    (particle.position[dim_0] * zoom + (SCREEN_WIDTH as f64 / 2f64)) as isize + shift.0,
                    (particle.position[dim_1] * zoom + (SCREEN_HEIGHT as f64 / 2f64)) as isize + shift.1,
                    size,
                    size,
                    particle_color,
                );
            } else {
                framebuffer.draw_circle(
                    (particle.position[dim_0] * zoom + (SCREEN_WIDTH as f64 / 2f64)) as isize + shift.0,
                    (particle.position[dim_1] * zoom + (SCREEN_HEIGHT as f64 / 2f64)) as isize + shift.1,
                    size,
                    particle_color,
                );
            }
        }
        if clear_between_frames {
            framebuffer.draw_cross(
                mouse_position.0 as isize,
                mouse_position.1 as isize,
                &[255, 255, 255, 255],
            );
        }
        total_rendering_time += start.elapsed();

        let start = Instant::now();
        framebuffer.draw();
        total_drawing_time += start.elapsed();

        let update_duration = update_start.elapsed();
        if update_duration < DESIRED_UPDATE_DURATION && limit_ups {
            sleep(DESIRED_UPDATE_DURATION - update_duration);
        }
    }

    unsafe {
        libc::ioctl(tty.as_raw_fd(), KDSETMODE, KD_TEXT);
    }

    println!("UPS: {}", i as f32 / engine_start_instant.elapsed().as_secs() as f32);
    println!("Total time: {}ms", engine_start_instant.elapsed().as_millis());
    println!("Simulation time: {}ms", total_simulation_time.as_millis());
    println!("Rendering time: {}ms", total_rendering_time.as_millis());
    println!("Drawing time: {}ms", total_drawing_time.as_millis());
    println!("Clearing screen time: {}ms", total_clearing_screen_time.as_millis());
    println!("Input handling time: {}ms", total_input_handling_time.as_millis());
    println!(
        "Unaccounted time: {}ms",
        (engine_start_instant.elapsed()
            - total_simulation_time
            - total_rendering_time
            - total_drawing_time
            - total_input_handling_time
            - total_clearing_screen_time)
            .as_millis()
    );
}
