use image::imageops::FilterType;
use image::DynamicImage;
use image::ImageFormat;
use std::io::Cursor;

pub fn resize(
    image: &DynamicImage,
    image_fmt: ImageFormat,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, String> {
    let filter_type = if needs_upscale(&image, width, height) {
        FilterType::CatmullRom
    } else {
        FilterType::Lanczos3
    };

    let new_image = image.resize(width, height, filter_type);

    let mut bytes: Vec<u8> = Vec::new();
    new_image
        .write_to(&mut Cursor::new(&mut bytes), image_fmt)
        .map_err(|e| e.to_string())?;

    Ok(bytes)
}

fn needs_upscale(image: &DynamicImage, width: u32, height: u32) -> bool {
    if image.width() >= width {
        return true;
    };
    if image.height() >= height {
        return true;
    };
    false
}
