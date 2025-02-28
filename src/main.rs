use eframe::egui;
use rfd::FileDialog;
use image::{Luma, ImageReader};
use std::time::Instant;

const ASCII_CHARS: &[u8] = b"@%#MW&8B$*o!;:. ";
const WIDTH: usize = 120;
const HEIGHT: usize = 40;

struct MyApp {
    selected_file: Option<String>,
    rotation_axis: String,
    ascii_art: Option<Vec<Vec<char>>>,
    angle_x: f64,
    angle_y: f64,
    angle_z: f64,
    last_update: Instant,
    rotation_speed: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_file: None,
            rotation_axis: "z".to_string(),
            ascii_art: None,
            angle_x: 0.0,
            angle_y: 0.0,
            angle_z: 0.0,
            last_update: Instant::now(),
            rotation_speed: 0.05,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Select Image").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        self.selected_file = Some(path.to_string_lossy().to_string());
                        self.load_ascii_art();
                    }
                }
                if let Some(file) = &self.selected_file {
                    ui.label(format!("File: {}", file));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Rotation Axis:");
                if ui.button("X").clicked() {
                    self.rotation_axis = "x".to_string();
                    self.reset_rotation();
                }
                if ui.button("Y").clicked() {
                    self.rotation_axis = "y".to_string();
                    self.reset_rotation();
                }
                if ui.button("Z").clicked() {
                    self.rotation_axis = "z".to_string();
                    self.reset_rotation();
                }
            });

            ui.label(format!("Current Rotation Axis: {}", self.rotation_axis));
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Rotation Speed:");
                ui.add(egui::Slider::new(&mut self.rotation_speed, 0.01..=0.5).text("Speed"));
            });
            
            if let Some(ascii) = self.ascii_art.as_ref() {
                let rotated_ascii = self.rotate_ascii(ascii);
                ui.add_space(10.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for row in rotated_ascii.iter() {
                        ui.monospace(row.iter().collect::<String>());
                    }
                });
            }
        });
        self.smooth_update_rotation();
    }
}

impl MyApp {
    fn load_ascii_art(&mut self) {
        if let Some(path) = &self.selected_file {
            let img = ImageReader::open(path).unwrap().decode().unwrap().into_luma8();
            let resized = image::imageops::resize(&img, WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Gaussian);
            self.ascii_art = Some(resized.pixels()
                .map(|Luma([b])| {
                    if *b > 240 { ' ' } else { ASCII_CHARS[(*b as usize * (ASCII_CHARS.len() - 1) / 255) as usize] as char }
                })
                .collect::<Vec<_>>()
                .chunks(WIDTH)
                .map(|row| row.to_vec())
                .collect());
        }
    }

    fn reset_rotation(&mut self) {
        self.angle_x = 0.0;
        self.angle_y = 0.0;
        self.angle_z = 0.0;
        self.last_update = Instant::now();
    }

    fn smooth_update_rotation(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        let delta = elapsed.as_secs_f64();
        self.last_update = now;

        let speed_factor = self.rotation_speed * delta * 60.0;
        match self.rotation_axis.as_str() {
            "x" => self.angle_x += speed_factor,
            "y" => self.angle_y += speed_factor,
            "z" => self.angle_z += speed_factor,
            _ => (),
        }
    }

    fn rotate_ascii(&self, ascii_art: &[Vec<char>]) -> Vec<Vec<char>> {
        let mut rotated = vec![vec![' '; WIDTH]; HEIGHT];
        let cx = (WIDTH / 2) as f64;
        let cy = (HEIGHT / 2) as f64;
        let cos_x = self.angle_x.cos();
        let sin_x = self.angle_x.sin();
        let cos_y = self.angle_y.cos();
        let sin_y = self.angle_y.sin();
        let cos_z = self.angle_z.cos();
        let sin_z = self.angle_z.sin();

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
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ASCII 3D Rotation",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
