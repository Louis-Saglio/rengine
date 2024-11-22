use crate::physics::Population;
use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::mem::transmute;

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
    
    fn get_buffer_index(x: usize, y: usize) -> usize {
        (y * SCREEN_WIDTH + x) * BYTES_PER_PIXEL
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: &[u8; BYTES_PER_PIXEL]) {
        let offset = Framebuffer::get_buffer_index(x, y);
        let pixel_slice = &mut self.mmap[offset..offset + BYTES_PER_PIXEL];
        pixel_slice.copy_from_slice(color);
    }
    
    pub fn draw_square(&mut self, x: usize, y: usize, width: usize, height: usize, color: [u8; BYTES_PER_PIXEL]) {
        for row_index in 0..height {
            let anchor_index = Framebuffer::get_buffer_index(x + row_index, y);
            let slice = &mut self.mmap[anchor_index..((anchor_index + width) * BYTES_PER_PIXEL)];
            slice.copy_from_slice(&vec![color; width].into_flattened());
        }
    }
}

pub fn sandbox(population: Population) {
    let mut framebuffer = Framebuffer::new();
    loop {
        framebuffer.clear();
        for particle in population.iter() {
            println!("{:?}", particle.position);
            framebuffer.draw_square(
                (particle.position[0] + 500.0) as usize,
                (particle.position[1] + 500.0) as usize,
                20,
                20,
                [255, 255, 255, 255],
            )
        }
        break
    }
}
