use image::{DynamicImage, GenericImageView, GenericImage};

pub fn create_mosaic(image: DynamicImage, normalized_images: Vec<DynamicImage>, x_dim: u32, y_dim: u32) -> DynamicImage {
    // create new image of size x_dim, y_dim
    let mut mosaic = DynamicImage::new_rgb8(x_dim, y_dim);
    // for each pixel in image, find closest image in normalized_images
    // and set pixel in mosaic to that image
    let total_pixels = x_dim * y_dim;
    let mut pixels_processed = 0;
    for (x, y, pixel) in image.pixels() {
        if pixels_processed % 100 == 0 {
            println!("Processed {} of {} pixels", pixels_processed, total_pixels);
        }
        let r = pixel.0[0];
        let g = pixel.0[1];
        let b = pixel.0[2];
        let mut min_dist = 255 * 255 * 3;
        let mut min_index = 0;
        for (i, img) in normalized_images.iter().enumerate() {
            let pixel = img.get_pixel(x, y).0;
            let r2 = pixel[0];
            let g2 = pixel[1];
            let b2 = pixel[2];
            let dist = (r2 as i32 - r as i32).pow(2) + (g2 as i32 - g as i32).pow(2) + (b2 as i32 - b as i32).pow(2);
            if dist < min_dist {
                min_dist = dist;
                min_index = i;
            }
        }
        mosaic.put_pixel(x, y, normalized_images[min_index].get_pixel(x, y));
        pixels_processed += 1;
    }
    return mosaic
}