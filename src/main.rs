#[macro_use]
extern crate clap;

use std::fs::{self, File};
use image;
use clap::App;

fn scramble_file(path: &str) -> std::io::Result<()> {
    println!("Removing exif data for {}", path);

    // Read image data
    let im = image::open(path).unwrap().flipv().fliph();
    let format = image::ImageFormat::from_path(path).expect("Error reading image format");

    // Create the new file
    let mut out_file = File::create(path).unwrap();
    // Write image data
    match im.write_to(&mut out_file, format) {
        Ok(_) => Ok(()),
        Err(e) => panic!("Error: {}", e)
    }
}

fn main() -> std::io::Result<()>{
    let args = load_yaml!("arguments.yaml");
    let matches = App::from_yaml(args).get_matches();

    // Validate input
    assert!(matches.is_present("PATH"));
    let path = matches.value_of("PATH").unwrap();
    assert!(fs::metadata(path).is_ok());
    assert!(fs::metadata(path).unwrap().is_file());

    // Remove exif data
    match scramble_file(path) {
        Ok(_0) => Ok(()),
        Err(e) => panic!("Error: {}", e),
    }
}