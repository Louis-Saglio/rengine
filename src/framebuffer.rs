use crate::physics::{apply_force, Particle};
use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::thread::sleep;
use std::time::{Duration, Instant};

const BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH: usize = 1920;
const SCREEN_HEIGHT: usize = 1080;

const FRAMEBUFFER_LENGTH: usize = SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL;

const DESIRED_UPS: u8 = 60;
const DESIRED_UPDATE_DURATION: Duration = Duration::from_micros(1000000 / DESIRED_UPS as u64);

struct Framebuffer {
    mmap: MmapMut,
}

impl Framebuffer {
    pub fn new() -> Self {
        let file = OpenOptions::new().read(true).write(true).open("/dev/fb0").unwrap();
        let mmap = unsafe { MmapOptions::new().len(FRAMEBUFFER_LENGTH).map_mut(&file).unwrap() };
        Framebuffer { mmap }
    }

    pub fn clear(&mut self) {
        self.mmap.fill(0)
    }

    fn get_buffer_index(x: isize, y: isize) -> isize {
        (y * (SCREEN_WIDTH as isize) + x) * (BYTES_PER_PIXEL as isize)
    }

    pub fn draw_pixel(&mut self, x: isize, y: isize, color: &[u8; BYTES_PER_PIXEL]) {
        let anchor_pixel_index = Framebuffer::get_buffer_index(x, y);
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

pub fn sandbox() {
    let mut framebuffer = Framebuffer::new();
    let mut population = [
        Particle {
            mass: 100.0,
            speed: [0.0, 0.0],
            position: [500.0, 500.0],
        },
        Particle {
            mass: 100.0,
            speed: [0.0, 0.0],
            position: [600.0, 600.0],
        },
    ];
    println!("{:?}", population);
    for i in 0..1000 {
        let update_start = Instant::now();
        population = apply_force(&population);
        framebuffer.clear();
        println!("{:?}", population);
        for particle in population.iter() {
            framebuffer.draw_circle(
                particle.position[0] as isize,
                particle.position[1] as isize,
                5,
                &[255, 150, 100, 255],
            );
        }
        let update_duration = update_start.elapsed();
        if update_duration < DESIRED_UPDATE_DURATION {
            sleep(DESIRED_UPDATE_DURATION - update_duration);
        }
    }
}
