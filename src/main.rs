use std::{fs, path::PathBuf};
use rayon::prelude::*;

use image::GenericImageView;

mod photomosaic;
use photomosaic::create_mosaic;

fn get_all_images(directory: &str) -> Vec<image::DynamicImage> {
    println!("--- Reading images in directory. ---");
    let paths: Vec<PathBuf> = fs::read_dir(directory)
        .expect("Failed to read directory.")
        .map(|entry| entry.expect("Failed to read entry.").path())
        .collect();

    println!("--- Opening images, this might take a while. ---");
    let images: Vec<image::DynamicImage> = paths
        .par_iter()
        .map(|path| {
            return image::open(path).expect("Failed to open image.");
        })
        .collect();
    println!("--- Done reading images. ---");
    images
}

fn normalize(images: Vec<image::DynamicImage>, x_dim: u32, y_dim: u32) -> Vec<image::DynamicImage> {
    let mut normalized_images: Vec<image::DynamicImage> = Vec::new();
    for img in images {
        let normalized_img = img.resize(x_dim, y_dim, image::imageops::FilterType::Nearest);
        normalized_images.push(normalized_img);
    }
    return normalized_images;
}

fn main() {
    println!("Hello, world!");
    // READING INPUT FOR NUMBER
    let mut input = String::new();

    let number: i32 = loop {
        input.clear();

        println!("Enter a number: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from STDIN.");

        let input = input.trim();

        match input.parse() {
            Ok(i) => break i,
            Err(_) => eprintln!("`{input}` is not an integer."),
        }
    };
    println!("You asked for {} tiles", number);

    let image = image::open("to_mosaic/image.jpg").expect("File not found!");
    let (w, h) = image.dimensions();
    println!("Image dimensions: {} x {}", w, h);
    let x_dim = w / number as u32;
    let y_dim = h / number as u32;
    println!("x_dim: {}, y_dim: {}", x_dim, y_dim);
    
    let images = get_all_images("src_images/");
    if images.len() == 0 {
        println!("No images found in src_images/ directory");
        return;
    }
    println!("--- Normalizing images ---");
    let normalized_images = normalize(images, x_dim, y_dim);
    println!("--- Creating mosaic ---");
    let src_dim = [w, h];
    let mosaic = create_mosaic(image, src_dim, normalized_images, x_dim, y_dim);
    mosaic.save("mosaic.jpg").unwrap();
}
