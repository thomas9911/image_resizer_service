use image::imageops::overlay;
use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageFormat;

use crate::proto::ResizeMethod;
use std::io::Cursor;

pub fn resize(
    image: &DynamicImage,
    image_fmt: ImageFormat,
    width: u32,
    height: u32,
    method: ResizeMethod,
) -> Result<Vec<u8>, String> {
    let filter_type = if needs_upscale(image, width, height) {
        FilterType::CatmullRom
    } else {
        FilterType::Lanczos3
    };

    dbg!(&method);

    let new_image = match method {
        ResizeMethod::Fill => image.resize_to_fill(width, height, filter_type),
        ResizeMethod::Fit => image.resize(width, height, filter_type),
        ResizeMethod::Pad => {
            // we create a new image with the width and height and place the resized
            // image in the center of the new image
            let sub_image = image.resize(width, height, filter_type);
            let (sub_width, sub_height) = sub_image.dimensions();

            let width_placement = width.saturating_sub(sub_width as u32) / 2;
            let height_placement = height.saturating_sub(sub_height as u32) / 2;

            let mut out_image = image::DynamicImage::new_rgba8(width, height);
            overlay(
                &mut out_image,
                &sub_image,
                width_placement as i64,
                height_placement as i64,
            );
            out_image
        }
        ResizeMethod::Limit => unimplemented!(), // this stretches the image => image.resize_exact(width, height, filter_type),
    };

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
