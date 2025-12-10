use std::borrow::Cow;
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::Path;

fn main() {
    // Read the ICC profile from the file
    let icc_path = "target/example.icc";
    let icc_profile = if Path::new(icc_path).exists() {
        let mut file = File::open(icc_path).expect("Failed to open ICC profile");
        let mut profile = Vec::new();
        file.read_to_end(&mut profile).expect("Failed to read ICC profile");
        Some(Cow::Owned(profile))
    } else {
        println!("Warning: ICC profile not found at {}", icc_path);
        println!("Run the 'first' example with a PNG containing an ICC profile first.");
        None
    };

    // Create output file
    let output_path = "target/checkerboard.png";
    let file = File::create(output_path).expect("Failed to create output file");
    let writer = BufWriter::new(file);

    // Set up PNG info with ICC profile
    let width = 512;
    let height = 512;
    let mut info = png::Info::with_size(width, height);
    info.color_type = png::ColorType::Rgb;
    info.bit_depth = png::BitDepth::Eight;
    info.icc_profile = icc_profile.clone();

    // Create encoder with info
    let encoder = png::Encoder::with_info(writer, info).expect("Failed to create encoder");
    
    if icc_profile.is_some() {
        println!("ICC profile attached to PNG");
    }

    let mut writer = encoder.write_header().expect("Failed to write PNG header");

    // Create checkerboard pattern
    let checker_size = 64; // Size of each checker square
    let mut image_data = Vec::with_capacity((width * height * 3) as usize);

    for y in 0..height {
        for x in 0..width {
            // Determine if this pixel is in a "white" or "black" square
            let checker_x = (x / checker_size) % 2;
            let checker_y = (y / checker_size) % 2;
            let is_white = (checker_x + checker_y) % 2 == 0;

            // RGB values
            let color = if is_white { 255 } else { 0 };
            image_data.push(color); // R
            image_data.push(color); // G
            image_data.push(color); // B
        }
    }

    writer.write_image_data(&image_data).expect("Failed to write image data");

    println!("Checkerboard PNG created: {}", output_path);
    println!("Dimensions: {}x{}", width, height);
    println!("Checker square size: {}x{} pixels", checker_size, checker_size);
}
