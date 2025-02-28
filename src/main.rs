use image::{GrayImage, Luma, ImageReader};
use std::{thread, time, io};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::{stdout, Write};
use std::f64::consts::PI;

const ASCII_CHARS: &[u8] = b"@%#MW&8B$*o!;:. "; // More detailed ASCII gradient
const WIDTH: usize = 100; // Increased width for better detail
const HEIGHT: usize = 50; // Adjusted height for better aspect ratio
const ROTATION_SPEED: f64 = 0.1; // Rotation speed in radians

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
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

fn rotate_ascii(ascii_art: &[Vec<char>], angle_x: f64, angle_y: f64, angle_z: f64) -> Vec<Vec<char>> {
    let mut rotated = vec![vec![' '; WIDTH]; HEIGHT];
    let cx = (WIDTH / 2) as f64;
    let cy = (HEIGHT / 2) as f64;
    let cos_x = angle_x.cos();
    let sin_x = angle_x.sin();
    let cos_y = angle_y.cos();
    let sin_y = angle_y.sin();
    let cos_z = angle_z.cos();
    let sin_z = angle_z.sin();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;

            let nx = dx * cos_y - dy * sin_y;
            let ny = dx * sin_x * sin_y + dy * cos_x;
            let nz = dx * cos_x * sin_y - dy * sin_x;

            let final_x = (nx * cos_z - nz * sin_z + cx).round() as isize;
            let final_y = (ny + cy).round() as isize;

            if final_x >= 0 && final_x < WIDTH as isize && final_y >= 0 && final_y < HEIGHT as isize {
                rotated[final_y as usize][final_x as usize] = ascii_art[y][x];
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
    let image_path = get_user_input("Enter the path to the image file:");
    let img = load_image(&image_path);
    let ascii_art = image_to_ascii(&img);
    let mut angle_x = 0.0;
    let mut angle_y = 0.0;
    let mut angle_z = 0.0;
    let rotation_axis = get_user_input("Choose rotation axis (x, y, z):");
    
    loop {
        match rotation_axis.as_str() {
            "x" => angle_x += ROTATION_SPEED,
            "y" => angle_y += ROTATION_SPEED,
            "z" => angle_z += ROTATION_SPEED,
            _ => println!("Invalid axis, defaulting to z-axis rotation."),
        }

        if angle_x >= 2.0 * PI { angle_x = 0.0; }
        if angle_y >= 2.0 * PI { angle_y = 0.0; }
        if angle_z >= 2.0 * PI { angle_z = 0.0; }

        let rotated_ascii = rotate_ascii(&ascii_art, angle_x, angle_y, angle_z);
        display_ascii(&rotated_ascii);
        thread::sleep(time::Duration::from_millis(100));
    }
}
