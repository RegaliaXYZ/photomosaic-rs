use image::{DynamicImage, GenericImageView, GenericImage, Pixel};

pub fn create_mosaic(image: DynamicImage, src_dim: [u32;2], normalized_images: Vec<DynamicImage>, x_dim: u32, y_dim: u32) -> DynamicImage {
    // create new image of size x_dim, y_dim
    println!("--- Creating mosaic. ---");
    println!("src x: {}, srx y: {}, xdim: {}, ydim: {} --- ", src_dim[0], src_dim[1], x_dim, y_dim);
    let mut mosaic = DynamicImage::new_rgb8(src_dim[0], src_dim[1]);
    // for each pixel in image, find closest image in normalized_images
    // and set pixel in mosaic to that image
    let mut normalized_images_average_rgb = Vec::new();
    // calculate average rgb for each source image
    for image in &normalized_images {
        let average_rgb = average_rgb(image);
        normalized_images_average_rgb.push(average_rgb);
    }
    //let mut index_rgb = Vec::new();
    let list_src_cut = cut_img(image, x_dim, y_dim);
    println!("list_src_cut len: {}", list_src_cut.len());

    // for each image in list_src_cut, find closest image in normalized_images and paste it in mosaic
    let mut index_rgb = Vec::new();
    for src_cut in list_src_cut.iter() {
        let average_rgb = average_rgb(src_cut);
        let index = calcul_diff(average_rgb, normalized_images_average_rgb.to_owned());
        index_rgb.push(index);
    }
    println!("index_rgb len: {}", index_rgb.len());
    println!("{:?}", index_rgb);
    // for each index in index_rgb, paste the corresponding image in mosaic
    let mut i = 0;
    let mut pos = [0, 0];
    for y in (0..src_dim[1]).step_by(y_dim as usize) {
        for x in (0..src_dim[0]).step_by(x_dim as usize) {
            let index = index_rgb[i];
            println!("index: {}", index);
            let src = normalized_images[index as usize].to_owned();
            print!("x: {}, y: {}, ", x, y);
            mosaic.copy_from(&src, pos[0], pos[1]).expect("Failed to copy image");
            pos[0] = x;
            pos[1] = y;
            i += 1;
        }
    }
    return mosaic
}


fn cut_img(image: DynamicImage, x_dim: u32, y_dim: u32) -> Vec<DynamicImage> {
    let (width, height) = image.dimensions();
    let mut list_img = Vec::new();

    for y in (0..height).step_by(y_dim as usize) {
        for x in (0..width).step_by(x_dim as usize) {
            let right = (x + x_dim).min(width);
            let bottom = (y + y_dim).min(height);

            let cropped = image.crop_imm(x, y, right - x, bottom - y).to_owned();
            list_img.push(cropped);
        }
    }

    list_img
}

fn calcul_diff(average_rgb: [u8; 3], all_rgb: Vec<[u8; 3]>) -> u32 {
    // find the closest rgb in all_rgb to average_rgb and return index
    let mut min_dist = 255 * 255 * 3;
    let mut min_index = 0;
    for (i, rgb) in all_rgb.iter().enumerate() {
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];
        let dist = (r as i32 - average_rgb[0] as i32).pow(2) + (g as i32 - average_rgb[1] as i32).pow(2) + (b as i32 - average_rgb[2] as i32).pow(2);
        if dist < min_dist {
            min_dist = dist;
            min_index = i;
        }
    }
    return min_index as u32;
}

fn average_rgb(image: &DynamicImage) -> [u8; 3] {
    let (width, height) = image.dimensions();
    let count = width * height;

    let (r_sum, g_sum, b_sum) = image
        .pixels()
        .fold((0, 0, 0), |acc, (_, _, pixel)| {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            (acc.0 + r, acc.1 + g, acc.2 + b)
    });

    [
        (r_sum as u32 / count) as u8,
        (g_sum as u32 / count) as u8,
        (b_sum as u32 / count) as u8,
    ]
}