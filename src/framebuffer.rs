use crate::physics::{apply_force, Particle, POP_SIZE};
use memmap2::{MmapMut, MmapOptions};
use rand::{random, Rng};
use std::array;
use std::fs::OpenOptions;
use std::io::Read;
use std::mem::transmute;
use std::os::unix::fs::OpenOptionsExt;
use std::thread::sleep;
use std::time::{Duration, Instant};

const BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH: usize = 1920;
const SCREEN_HEIGHT: usize = 1080;

const FRAMEBUFFER_LENGTH: usize = SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL;

const DESIRED_UPS: u8 = 255;
const DESIRED_UPDATE_DURATION: Duration = if DESIRED_UPS == 0 {
    Duration::ZERO
} else {
    Duration::from_micros(1000000 / DESIRED_UPS as u64)
};

struct Framebuffer {
    mmap: MmapMut,
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
        Framebuffer { mmap }
    }

    pub fn clear(&mut self) {
        self.mmap.fill(0)
    }

    pub fn draw_pixel(&mut self, x: isize, y: isize, color: &[u8; BYTES_PER_PIXEL]) {
        let anchor_pixel_index = (y * (SCREEN_WIDTH as isize) + x) * (BYTES_PER_PIXEL as isize);
        if anchor_pixel_index >= 0 && anchor_pixel_index + (BYTES_PER_PIXEL as isize) < (FRAMEBUFFER_LENGTH as isize) {
            let anchor_pixel_index = anchor_pixel_index as usize;
            let pixel_slice = &mut self.mmap[anchor_pixel_index..anchor_pixel_index + BYTES_PER_PIXEL];
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
                    if (px as usize) < SCREEN_WIDTH && (py as usize) < SCREEN_HEIGHT {
                        self.draw_pixel(px, py, color);
                    }
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
struct InputEvent {
    time: [u64; 2],
    type_: u16,
    code: u16,
    value: i32,
}

pub fn run() {
    let mut kb_file = OpenOptions::new()
        .read(true)
        .custom_flags(0x800)
        .open("/dev/input/event10")
        .expect("Unable to open keyboard device");

    let mut mouse_file = OpenOptions::new()
        .read(true)
        .custom_flags(0x800)
        .open("/dev/input/event8")
        .expect("Unable to open mouse device");

    let mut framebuffer = Framebuffer::new();

    let mut population = Particle::new_random_pop_in_screen(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    let particles_colors: [[u8; BYTES_PER_PIXEL]; POP_SIZE] = array::from_fn(|_| random());

    let mut zoom: f64 = 1.0;
    let mut shift: (isize, isize) = (0, 0);

    loop {
        let update_start = Instant::now();

        let mut kb_buffer = [0u8; 24];
        match kb_file.read(&mut kb_buffer) {
            Ok(_) => {
                let kb_event: InputEvent = unsafe { transmute(kb_buffer) };
                if kb_event.type_ == 1 {
                    match kb_event.code {
                        105 => shift.0 += 10,
                        106 => shift.0 -= 10,
                        103 => shift.1 += 10,
                        108 => shift.1 -= 10,
                        _ => {}
                    }
                }
            }
            Err(_) => {}
        }

        let mut mouse_buffer = [0u8; 24];
        match mouse_file.read(&mut mouse_buffer) {
            Ok(_) => {
                let mouse_event: InputEvent = unsafe { transmute(mouse_buffer) };
                if mouse_event.type_ == 2 && mouse_event.code == 8 {
                    match mouse_event.value {
                        1 => zoom *= 1.1,
                        -1 => zoom *= 0.9,
                        _ => {}
                    }
                }
            }
            Err(_) => {}
        }

        population = apply_force(&population);

        framebuffer.clear();

        for (particle, particle_color) in population.iter().zip(particles_colors.iter()) {
            if particle.mass == 0.0 {
                continue;
            }
            framebuffer.draw_circle(
                (particle.position[0] * zoom + (SCREEN_WIDTH as f64 / 2f64)) as isize + shift.0,
                (particle.position[1] * zoom + (SCREEN_HEIGHT as f64 / 2f64)) as isize + shift.1,
                particle.mass.sqrt() as usize,
                particle_color,
            );
        }

        let update_duration = update_start.elapsed();
        if DESIRED_UPDATE_DURATION.is_zero() {
        } else if update_duration < DESIRED_UPDATE_DURATION {
            sleep(DESIRED_UPDATE_DURATION - update_duration);
        } else {
            println!("Update lasted {:?} too long", update_duration - DESIRED_UPDATE_DURATION);
        }
    }
}
