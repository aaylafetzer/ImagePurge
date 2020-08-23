use std::fs::{self, DirEntry, File};
use std::path::Path;
use exif::{Field, In, Tag, Value};
use exif::experimental::Writer;
use std::collections::HashMap;

fn scramble_file(path: &str) -> std::io::Result<()> {
    println!("Scrambling exif data for {}", path);

    // Store metadata to write to image as a HashMap to avoid creating a new variable for each one
    let mut image_data: Vec<Field> = Vec::new();
    image_data.push(Field { tag: Tag::ImageDescription, ifd_num: In::PRIMARY,
        value: Value::Ascii(vec![b"Your mom gay".to_vec()])
    });
    image_data.push(Field { tag: Tag::CameraOwnerName, ifd_num: In::PRIMARY,
        value: Value::Ascii(vec![b"Your future boyfriend".to_vec()])
    });
    image_data.push(Field { tag: Tag::GPSLatitude, ifd_num: In::PRIMARY,
        value: Value::Float(vec![37.2431].to_vec())
    });
    image_data.push(Field { tag: Tag::GPSLongitude, ifd_num: In::PRIMARY,
        value: Value::Float(vec![115.792999].to_vec())
    });
    image_data.push(Field {tag: Tag::Artist, ifd_num: In::PRIMARY,
        value: Value::Ascii(vec![b"The International Proletariat".to_vec()])
    });
    image_data.push(Field {tag: Tag::Copyright, ifd_num: In::PRIMARY,
        value: Value::Ascii(vec![b"Hippity Hoppity Abolish Private Property".to_vec()])
    });
    image_data.push(Field {tag: Tag::Make, ifd_num: In::PRIMARY,
        value: Value::Ascii(vec![b"Deez".to_vec()])
    });
    image_data.push(Field {tag: Tag::Model, ifd_num: In::PRIMARY,
        value: Value::Ascii(vec![b"Nuts".to_vec()])
    });
    image_data.push(Field {tag: Tag::Software, ifd_num: In::PRIMARY,
        value: Value::Ascii(vec![b"".to_vec()])
    });
    // Create the new file
    let mut writer = Writer::new();
    let mut out_file = File::create(path).unwrap();

    // Push the custom metadata to writer
    for i in image_data.iter() {
        writer.push_field(i);
    }
    // Write exif to file
    writer.write(&mut out_file, false);
    Ok(())
}

fn check_path(path: &str) -> std::io::Result<()> {
    let meta = fs::metadata(path);
    match meta {  // Test if path actually exists
        Ok(meta) => {
            if meta.is_dir() {
                println!("Directory Input");
            } else if meta.is_file() {
                scramble_file(path);
            } else {
                panic!("Error: path is not a file or a directory!")
            }
        }
        Err(e) => panic!("Error: {}", e)  // The path does not exist
    };
    Ok(())
}

fn main() {
    check_path("/home/aayla/Pictures/photo.jpg");
}
