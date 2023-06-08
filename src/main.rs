use image::GenericImageView;

fn main() {
    println!("Hello, world!");

    loop {
        let mut input = String::new();
        println!("Enter an even number: ");
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let number = match input.trim().parse::<i32>() {
            Ok(i) => i,
            Err(..) => {
                println!("This was not an integer: {}", input.trim());
                continue;
            }
        };

        if number % 2 == 0 {
            break;
        } else {
            println!("This was not an even number: {}", number);
        }
    }

    let image = image::open("to_mosaic/image.jpg").expect("File not found!");

    let (w, h) = image.dimensions();
    println!("Image dimensions: {} x {}", w, h);

    println!("There are {} pixels", image.pixels().count());
}
