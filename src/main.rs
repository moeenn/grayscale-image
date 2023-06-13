mod image_utils;
mod timer;

fn main() {
    let images: Vec<String> = std::env::args().skip(1).collect();
    for image in &images {
        let is_valid = image_utils::is_valid_image(image);
        if !is_valid {
            println!("error: invalid image path: {}", image);
            std::process::exit(1);
        }
    }

    let t = timer::Timer::new("process_images");
    let mut count = 0;

    for input_image in &images {
        let result =
            image_utils::image_to_grayscale(input_image, &image_utils::output_image_name(count));

        match result {
            Ok(_) => println!("saved: {}", input_image),
            Err(e) => {
                println!("error: {}, {}", input_image, e);
            }
        };

        count += 1;
    }

    t.elapsed();
}
