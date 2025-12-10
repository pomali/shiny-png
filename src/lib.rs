use std::borrow::Cow;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// Static ICC profile embedded in the library
const DEFAULT_ICC_PROFILE: &[u8] = include_bytes!("../example.icc");

/// Apply an ICC profile to a PNG image
///
/// # Arguments
/// * `image_data` - The PNG image data as bytes
/// * `icc_profile` - The ICC profile data as bytes
///
/// # Returns
/// A new PNG image with the ICC profile embedded
#[wasm_bindgen]
pub fn apply_icc_profile(image_data: &[u8], icc_profile: &[u8]) -> Result<Vec<u8>, JsValue> {
    // Decode the input PNG
    let decoder = png::Decoder::new(Cursor::new(image_data));
    let mut reader = decoder.read_info().map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Get the image info
    let mut info = reader.info().clone();
    
    // Set the ICC profile
    info.icc_profile = Some(Cow::Owned(icc_profile.to_vec()));

    // Read the image data
    let mut buf = vec![0; reader.output_buffer_size().unwrap()];
    reader.next_frame(&mut buf).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Encode with the new ICC profile
    let mut output = Vec::new();
    {
        let encoder = png::Encoder::with_info(&mut output, info)
            .map_err(|e| JsValue::from_str(&format!("Encoder error: {:?}", e)))?;
        
        let mut writer = encoder.write_header()
            .map_err(|e| JsValue::from_str(&format!("Header error: {:?}", e)))?;
        
        writer.write_image_data(&buf)
            .map_err(|e| JsValue::from_str(&format!("Write error: {:?}", e)))?;
    }

    Ok(output)
}

/// Extract ICC profile from a PNG image
///
/// # Arguments
/// * `image_data` - The PNG image data as bytes
///
/// # Returns
/// The ICC profile data if present, or an empty vector if not
#[wasm_bindgen]
pub fn extract_icc_profile(image_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    let decoder = png::Decoder::new(Cursor::new(image_data));
    let reader = decoder.read_info().map_err(|e| JsValue::from_str(&e.to_string()))?;

    let info = reader.info();
    
    if let Some(profile) = &info.icc_profile {
        Ok(profile.to_vec())
    } else {
        Ok(Vec::new())
    }
}

/// Apply ICC profile from one image to another
///
/// # Arguments
/// * `source_image` - The PNG image to extract the ICC profile from
/// * `target_image` - The PNG image to apply the ICC profile to
///
/// # Returns
/// The target image with the ICC profile from the source image
#[wasm_bindgen]
pub fn transfer_icc_profile(source_image: &[u8], target_image: &[u8]) -> Result<Vec<u8>, JsValue> {
    // Extract profile from source
    let icc_profile = extract_icc_profile(source_image)?;
    
    if icc_profile.is_empty() {
        return Err(JsValue::from_str("No ICC profile found in source image"));
    }

    // Apply to target
    apply_icc_profile(target_image, &icc_profile)
}

/// Apply the default embedded ICC profile to a PNG image
///
/// The library includes a pre-defined ICC profile that can be applied
/// to any PNG image without needing to provide an external profile.
///
/// # Arguments
/// * `image_data` - The PNG image data as bytes
///
/// # Returns
/// The PNG image with the default ICC profile embedded
#[wasm_bindgen]
pub fn apply_default_icc_profile(image_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    apply_icc_profile(image_data, DEFAULT_ICC_PROFILE)
}

/// Get the embedded default ICC profile
///
/// Returns a copy of the ICC profile that is statically included in the library.
///
/// # Returns
/// The default ICC profile data as bytes
#[wasm_bindgen]
pub fn get_default_icc_profile() -> Vec<u8> {
    DEFAULT_ICC_PROFILE.to_vec()
}
