use rand::Rng;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::process::exit;

const SCREEN_WIDTH: usize = 1920;
const SCREEN_HEIGHT: usize = 1080;
const BYTES_PER_PIXEL: usize = 4;

pub fn sandbox() {
    let fb_path = "/dev/fb0";
    let mut fb_file = match OpenOptions::new()
        .write(true)
        .read(true)
        // .custom_flags(libc::O_SYNC)
        .open(fb_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to open {}: {:?}", fb_path, e);
            exit(1);
        }
    };

    // Framebuffer characteristics

    // Create a buffer to represent the screen content
    for i in 0..255 {
        let mut buffer = vec![0x00; SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL];

        // Fill the screen with white color
        for pixel in buffer.chunks_mut(BYTES_PER_PIXEL) {
            pixel[0] = rand::thread_rng().gen_range(0..=255); // Blue
            pixel[1] = rand::thread_rng().gen_range(0..=255); // Green
            pixel[2] = rand::thread_rng().gen_range(0..=255); // Red
        }

        // Seek to the beginning of the framebuffer
        match fb_file.seek(SeekFrom::Start(0)) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Unable to seek in {}: {:?}", fb_path, e);
                exit(1);
            }
        }

        // Write the buffer to the framebuffer device
        match fb_file.write_all(&buffer) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Unable to write to {}: {:?}", fb_path, e);
                exit(1);
            }
        }

        println!("Framebuffer update successful!");
    }
}