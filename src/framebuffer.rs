use crate::physics::Population;
use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;

const BYTES_PER_PIXEL: usize = 4;
const SCREEN_WIDTH: usize = 1920;
const SCREEN_HEIGHT: usize = 1080;

struct Framebuffer {
    mmap: MmapMut,
}

impl Framebuffer {
    pub fn new() -> Self {
        let file = OpenOptions::new().read(true).write(true).open("/dev/fb0").unwrap();
        let mmap = unsafe {
            MmapOptions::new()
                .len(SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL)
                .map_mut(&file)
                .unwrap()
        };
        Framebuffer { mmap }
    }

    pub fn clear(&mut self) {
        self.mmap.fill(0)
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: &[u8; BYTES_PER_PIXEL]) {
        let offset = (y * SCREEN_WIDTH + x) * BYTES_PER_PIXEL;
        let pixel_slice = &mut self.mmap[offset..offset + BYTES_PER_PIXEL];
        pixel_slice.copy_from_slice(color);
    }
}

pub fn sandbox(population: Population) {
    let mut framebuffer = Framebuffer::new();
    framebuffer.clear();
    for particle in population.iter() {
        framebuffer.draw_pixel(
            particle.position[0] as usize,
            particle.position[1] as usize,
            &[255, 255, 255, 255],
        )
    }
}
