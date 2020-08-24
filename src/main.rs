#[macro_use]
extern crate clap;
extern crate rustface;

use std::fs::{File};
use image;
use clap::{App};
use rustface::{Detector, FaceInfo, ImageData};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use image::{Rgb, GrayImage, GenericImageView};

fn scramble_file(path: &str, hide_faces: bool) {
    // Read image data
    let im = image::open(path);
    if im.is_ok() {
        let image_data = im.unwrap();
        println!("Removing exif data for {}", path);
        let format = image::ImageFormat::from_path(path).expect("Error reading image format");
        let mut rgb = image_data.to_rgb();

        // Face Detection
        if hide_faces {
            println!("Hiding faces in {}", path);
            let mut detector = rustface::create_detector("rustface_model.bin").unwrap();
            detector.set_min_face_size(20);
            detector.set_score_thresh(1.0);
            detector.set_pyramid_scale_factor(0.5);
            detector.set_slide_window_step(1, 1);

            // Detect faces
            let gray = image_data.to_luma();
            let (width, height) = gray.dimensions();
            let mut rustface_image = ImageData::new(gray.as_ptr(), width, height);
            let mut i = 0;
            for face in detector.detect(&mut rustface_image) {
                i += 1;
                let bbox = face.bbox();
                let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());
                // Draw a rectangle
                draw_filled_rect_mut(&mut rgb, rect, Rgb([0, 0, 255]));
            }
            println!("Covered {} faces", i);
        }
        rgb.save(path);
    } else {
        println!("{} is not an image. Moving on to next file.", path);
    }
}

fn main() {
    let args = load_yaml!("arguments.yaml");
    let matches = App::from_yaml(args).get_matches();

    // Input validation and metadata checks
    assert!(matches.is_present("PATH"));

    let paths = matches.values_of("PATH").unwrap();

    // Split the paths into equally sized vectors for each thread
    for path in paths {
        scramble_file(path, matches.is_present("hide_faces"));
    }
}
