use std::error::Error;
use image::ImageOutputFormat;

pub fn process_image(input_bytes: Vec<u8>) -> Result<Vec<(String, Vec<u8>)>, Box<dyn Error>> {
    // Decode raw bytes into an image
    let img = image::load_from_memory(&input_bytes)?;

    // Create a 200x200 thumbnail version
    let thumb = img.thumbnail(200, 200);
    let mut thumb_bytes = std::io::Cursor::new(Vec::new());
    thumb.write_to(&mut thumb_bytes, ImageOutputFormat::Jpeg(80))?;

    // Resize to another version (800x800)
    let medium = img.resize(800, 800, image::imageops::FilterType::Lanczos3);
    let mut medium_bytes = std::io::Cursor::new(Vec::new());
    medium.write_to(&mut medium_bytes, ImageOutputFormat::Jpeg(80))?;

    // Return processed image buffers with suffixes
    Ok(vec![
        ("thumb".to_string(), thumb_bytes.into_inner()),
        ("medium".to_string(), medium_bytes.into_inner()),
    ])
}