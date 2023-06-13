use image::{self, ImageError};
use std::env;

pub fn image_to_grayscale<'a>(input: &'a str, output: &'a str) -> Result<(), ImageError> {
    let input_img = image::open(input)?;
    let output_img = input_img.grayscale();
    output_img.save(output)?;

    Ok(())
}

pub fn output_image_name(count: u32) -> String {
    let pwd = env::var("PWD").unwrap_or("/default".to_string());
    let pieces = pwd.split("/");
    let dir_name = pieces.last().unwrap_or("default");

    format!("{}_{:0>3}.jpg", dir_name, count)
}

pub fn is_valid_image(path: &String) -> bool {
    let allowed_formats = vec![".jpg", ".jpeg", ".png"];
    for format in allowed_formats {
        if path.to_lowercase().ends_with(format) {
            return true;
        }
    }
    false
}

#[test]
fn test_is_valid_image() {
    assert!(is_valid_image(&String::from("/some/path/image.jpg")));
    assert!(is_valid_image(&String::from("/not/valid/file.exe")) == false);
}
