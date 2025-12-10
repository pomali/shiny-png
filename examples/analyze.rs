use std::env;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() {
    // Get filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <png_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    // Open the file
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Create a decoder
    let decoder = png::Decoder::new(reader);
    let reader = decoder.read_info().expect("Failed to read PNG info");

    // Get the info
    let info = reader.info();

    println!("=== PNG Metadata ===\n");

    // Basic image information
    println!("Dimensions: {}x{}", info.width, info.height);
    println!("Bit Depth: {:?}", info.bit_depth);
    println!("Color Type: {:?}", info.color_type);
    println!("Interlaced: {}", if info.interlaced { "Yes" } else { "No" });
    println!("Bytes per pixel: {}", info.bytes_per_pixel());

    // Color information
    if let Some(palette) = &info.palette {
        println!("\n--- Palette ---");
        println!("Palette entries: {}", palette.len() / 3);
    }

    if let Some(trns) = &info.trns {
        println!("\n--- Transparency ---");
        println!("Transparency data length: {} bytes", trns.len());
    }

    // Gamma information
    if let Some(gamma) = info.source_gamma {
        println!("\n--- Gamma ---");
        println!("Source Gamma: {:?}", gamma);
    }

    // Chromaticity information
    if let Some(chroma) = info.source_chromaticities {
        println!("\n--- Chromaticities ---");
        println!("White point: ({:?}, {:?})", chroma.white.0, chroma.white.1);
        println!("Red: ({:?}, {:?})", chroma.red.0, chroma.red.1);
        println!("Green: ({:?}, {:?})", chroma.green.0, chroma.green.1);
        println!("Blue: ({:?}, {:?})", chroma.blue.0, chroma.blue.1);
    }

    // sRGB information
    if let Some(rendering_intent) = info.srgb {
        println!("\n--- sRGB ---");
        println!("Rendering Intent: {:?}", rendering_intent);
    }

    // ICC Profile
    if let Some(icc_profile) = &info.icc_profile {
        println!("\n--- ICC Profile ---");
        println!("Profile data length: {} bytes", icc_profile.len());

        // Write ICC profile to file
        let output_path = "target/example.icc";
        match File::create(output_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(icc_profile) {
                    eprintln!("Failed to write ICC profile: {}", e);
                } else {
                    println!("ICC profile written to: {}", output_path);
                }
            }
            Err(e) => {
                eprintln!("Failed to create output file: {}", e);
            }
        }

        println!("Profile data (first 32 bytes or full length if smaller):");
        for byte in icc_profile.iter().take(32) {
            print!("{:02X} ", byte);
        }
    }

    // Physical dimensions
    if let Some(pixel_dims) = info.pixel_dims {
        println!("\n--- Physical Dimensions ---");
        println!("Pixels per unit X: {}", pixel_dims.xppu);
        println!("Pixels per unit Y: {}", pixel_dims.yppu);
        println!("Unit: {:?}", pixel_dims.unit);
    }

    // Text chunks (metadata)
    if !info.uncompressed_latin1_text.is_empty() {
        println!("\n--- Uncompressed Latin-1 Text ---");
        for chunk in &info.uncompressed_latin1_text {
            println!(
                "{}: {}",
                chunk.keyword,
                String::from_utf8_lossy(chunk.text.as_bytes())
            );
        }
    }

    if !info.compressed_latin1_text.is_empty() {
        println!("\n--- Compressed Latin-1 Text ---");
        for chunk in &info.compressed_latin1_text {
            println!(
                "{}: {}",
                chunk.keyword,
                String::from_utf8_lossy(chunk.get_text().unwrap().as_bytes())
            );
        }
    }

    if !info.utf8_text.is_empty() {
        println!("\n--- UTF-8 Text ---");
        for chunk in &info.utf8_text {
            println!("{}: {}", chunk.keyword, chunk.get_text().unwrap());
        }
    }

    // Frame control (for APNG)
    if let Some(frame_control) = &info.frame_control {
        println!("\n--- Frame Control (APNG) ---");
        println!("Sequence number: {}", frame_control.sequence_number);
        println!("Width: {}", frame_control.width);
        println!("Height: {}", frame_control.height);
        println!("X offset: {}", frame_control.x_offset);
        println!("Y offset: {}", frame_control.y_offset);
        println!(
            "Delay: {}/{} seconds",
            frame_control.delay_num, frame_control.delay_den
        );
        println!("Dispose operation: {:?}", frame_control.dispose_op);
        println!("Blend operation: {:?}", frame_control.blend_op);
    }

    // Animation control (for APNG)
    if let Some(actl) = &info.animation_control {
        println!("\n--- Animation Control (APNG) ---");
        println!("Number of frames: {}", actl.num_frames);
        println!(
            "Number of plays: {}",
            if actl.num_plays == 0 {
                "infinite".to_string()
            } else {
                actl.num_plays.to_string()
            }
        );
    }

    println!("\n=== End of Metadata ===");
}
