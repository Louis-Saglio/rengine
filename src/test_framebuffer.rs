use std::fs::File;
use std::io::Read;

pub fn test_framebuffer() {
    let mut framebuffer = vec![0u8; 2560 * 1440 * 4];
    let mut file = File::open("/dev/fb0").unwrap();
    file.read_exact(&mut framebuffer).unwrap();
    let result = framebuffer.into_iter().map(|it| it as u128).sum::<u128>();
    println!("Result: {}", result);
}