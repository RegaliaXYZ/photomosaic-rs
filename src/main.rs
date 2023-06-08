use image::GenericImageView;

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
}
