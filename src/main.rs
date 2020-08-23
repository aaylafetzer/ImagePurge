#[macro_use]
extern crate clap;

use std::fs::{self, DirEntry, File};
use std::io::BufReader;
use image;
use clap::{Arg, App};

fn scramble_file(path: &str) -> std::io::Result<()> {
    println!("Removing exif data for {}", path);

    // Read image data
    let mut im = image::open(path).unwrap().flipv().fliph();
    // Create the new file
    let mut out_file = File::create(path).unwrap();
    // Write image data
    im.write_to(&mut out_file, image::ImageFormat::Jpeg);

    Ok(())
}

fn main() -> std::io::Result<()>{
    let args = load_yaml!("arguments.yaml");
    let matches = App::from_yaml(args).get_matches();

    // Validate input
    assert!(matches.is_present("PATH"));
    let path = matches.value_of("PATH").unwrap();
    let metadata = fs::metadata(path);
    assert!(fs::metadata(path).is_ok());
    assert!(fs::metadata(path).unwrap().is_file());

    // Remove exif data
    match scramble_file(path) {
        Ok(_0) => Ok(()),
        Err(e) => panic!("Error: {}", e),
    }
}