extern crate image;
extern crate time;
extern crate open;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use image::{DynamicImage, GenericImageView};
use time::PreciseTime;
use image::imageops::FilterType;


const ASCII_CHARS: [char; 11] = ['@', '#', '0', 'O', 'L', ';', ':', '.', ',', '\'', ' '];

fn convert_to_ascii(image: DynamicImage, resolution: u32) -> String {
    let mut ascii_art = String::new();
    let (width, height) = image.dimensions();
    let small_image = image.resize_exact(width / resolution, height / resolution, FilterType::Nearest);
    
    for y in 0..small_image.height() {
        for x in 0..small_image.width() {
            let pixel = small_image.get_pixel(x, y);
            let brightness = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
            let index = (brightness as usize * (ASCII_CHARS.len() - 1)) / 255;
            ascii_art.push(ASCII_CHARS[index]);
        }
        ascii_art.push('\n');
    }
    
    ascii_art
}

fn write_to_file(file_name: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <input_image_path> <resolution>", args[0]);
        return;
    }

    let image_path = &args[1];
    let resolution: u32 = args[2].parse().unwrap_or(5);

    let image = match image::open(image_path) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let start_time = PreciseTime::now();
    let ascii_art = convert_to_ascii(image, resolution);
    let end_time = PreciseTime::now();

    println!("Conversion completed in {} milliseconds", start_time.to(end_time).num_milliseconds());

    if let Err(err) = write_to_file("output.txt", &ascii_art) {
        eprintln!("Error writing to file: {}", err);
    } else {
        println!("ASCII art saved to output.txt");
    }
}

