use num_complex::Complex64;
use palette::{Hsl, rgb::Rgb};
use bmp::{Image, Pixel};
mod settings;
use settings::{MandelbrotSettings, DrawingSettings};

type ColorMap = std::collections::HashMap<u32, Pixel>;

fn main() {
    //print_to_console();
    let mandelbrot_settings = MandelbrotSettings {
        max_iterations: 1000,
        max_radius: 2.0
    };
    let drawing_settings = DrawingSettings {
        pixels_per_unit: 2000,
        window_x_min: -2,
        window_x_max: 1,
        window_y_min: -1,
        window_y_max: 1,
        color_hue_factor: 5
    };

    create_mandelbrot_image(mandelbrot_settings, drawing_settings);
}

fn create_mandelbrot_image(mandelbrot_settings: MandelbrotSettings, drawing_settings: DrawingSettings) {
    let max_iterations = mandelbrot_settings.max_iterations;
    let max_radius = mandelbrot_settings.max_radius as f64;

    let width_in_units = (drawing_settings.window_x_max - drawing_settings.window_x_min) as u32;
    let height_in_units = (drawing_settings.window_y_max - drawing_settings.window_y_min) as u32;
    let width_in_pixels = width_in_units * drawing_settings.pixels_per_unit;
    let height_in_pixels = height_in_units * drawing_settings.pixels_per_unit;
    let total_pixels = width_in_pixels * height_in_pixels;

    // f64 configs
    let pixels_per_unit = drawing_settings.pixels_per_unit as f64;
    let window_x_min = drawing_settings.window_x_min as f64;
    let window_x_max = drawing_settings.window_x_max as f64;
    let window_y_min = drawing_settings.window_y_min as f64;
    let window_y_max = drawing_settings.window_y_max as f64;

    let mut color_map = ColorMap::with_capacity(max_iterations as usize);

    // create bmp
    let mut img = Image::new(width_in_pixels, height_in_pixels);

    // iterate pixels
    for y in 0..height_in_pixels {
        for x in 0..width_in_pixels {
            let real = x as f64 / pixels_per_unit + window_x_min;
            let imaginary = -(y as f64) / pixels_per_unit + window_y_max;
            let complex = Complex64::new(real, imaginary);
            let (belongs, iterations) = belongs_to_mandelbrot_set(complex, max_iterations, max_radius);
            if !belongs {
                let pixel = get_colored_pixel(iterations, drawing_settings.color_hue_factor, &mut color_map);
                img.set_pixel(x, y, pixel);
            }
        }
    }

    img.save("mandelbrot.bmp").unwrap();
}

#[inline]
fn get_colored_pixel(iterations: u32, color_hue_factor: u32, color_map: &mut ColorMap) -> Pixel {
    *color_map.entry(iterations).or_insert_with(|| {
        let hue = (iterations * color_hue_factor) % 360;
        const SATURATION: f32 = 0.5;
        const LIGHTNESS: f32 = 0.5;
        let color: Rgb = Hsl::new(hue as f32, SATURATION, LIGHTNESS).into();
        Pixel::new((color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8)
    })
}

fn print_to_console() {
    for y in (-10..=10).rev() {
        for x in -40..=30 {
            let complex = Complex64::new(x as f64 / 20.0, y as f64 / 10.0);
            let mark = if belongs_to_mandelbrot_set(complex, 2000, 2.0).0 {
                "x"
            } else {
                " "
            };
            print!("{}", mark);
        }
        print!("\n");
    }
}

#[inline]
fn belongs_to_mandelbrot_set(complex: Complex64, max_iterations: u32, max_radius: f64) -> (bool, u32) {
    let mut iterations = 0;
    let mut result = complex;

    while iterations < max_iterations {
        iterations += 1;
        result = mandelbrot(complex, result);
        let (magnitude, _phase) = result.to_polar();
        if magnitude > max_radius {
            return (false, iterations);
        }
    }
    (true, iterations)
}

#[inline]
fn mandelbrot(initial: Complex64, current: Complex64) -> Complex64 {
    current.powi(2) + initial
}