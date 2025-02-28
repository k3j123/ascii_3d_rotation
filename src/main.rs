use image::{GrayImage, Luma, ImageReader};
use std::{thread, time, io};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::{stdout, Write};
use std::f64::consts::PI;

const ASCII_CHARS: &[u8] = b"@%#MW&8B$*o!;:. "; // More detailed ASCII gradient
const WIDTH: usize = 50; // Increased width for better detail
const HEIGHT: usize = 50; // Adjusted height for better aspect ratio
const ROTATION_SPEED: f64 = 0.1; // Rotation speed in radians

fn get_user_input() -> String {
    let mut input = String::new();
    println!("Enter the path to the image file:");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn load_image(path: &str) -> GrayImage {
    let img = ImageReader::open(path).unwrap().decode().unwrap().into_luma8();
    image::imageops::resize(&img, WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Gaussian)
}

fn image_to_ascii(img: &GrayImage) -> Vec<Vec<char>> {
    img.pixels()
        .map(|Luma([b])| {
            if *b > 240 { ' ' } else { ASCII_CHARS[(*b as usize * (ASCII_CHARS.len() - 1) / 255) as usize] as char } // More detailed mapping
        })
        .collect::<Vec<_>>()
        .chunks(WIDTH)
        .map(|row| row.to_vec())
        .collect()
}

fn rotate_ascii(ascii_art: &[Vec<char>], angle: f64) -> Vec<Vec<char>> {
    let mut rotated = vec![vec![' '; WIDTH]; HEIGHT];
    let cx = (WIDTH / 2) as f64;
    let cy = (HEIGHT / 2) as f64;
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let nx = ((x as f64 - cx) * cos_a - (y as f64 - cy) * sin_a + cx).round() as isize;
            let ny = ((x as f64 - cx) * sin_a + (y as f64 - cy) * cos_a + cy).round() as isize;
            if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                rotated[ny as usize][nx as usize] = ascii_art[y][x];
            }
        }
    }
    rotated
}

fn display_ascii(ascii_art: &[Vec<char>]) {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
    for (y, row) in ascii_art.iter().enumerate() {
        execute!(stdout, MoveTo(0, y as u16)).unwrap();
        writeln!(stdout, "{}", row.iter().collect::<String>()).unwrap();
    }
    stdout.flush().unwrap();
}

fn main() {
    let image_path = get_user_input();
    let img = load_image(&image_path);
    let ascii_art = image_to_ascii(&img);
    let mut angle = 0.0;
    
    loop {
        let rotated_ascii = rotate_ascii(&ascii_art, angle);
        display_ascii(&rotated_ascii);
        angle += ROTATION_SPEED;
        if angle >= 2.0 * PI { angle = 0.0; }
        thread::sleep(time::Duration::from_millis(100));
    }
}
