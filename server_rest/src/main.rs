use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::{Buf, Filter, Reply};

use std::io::{BufRead, Cursor, Read};

use warp::filters::multipart::{form, FormData};
use warp::http::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use warp::reply::with_header;
use warp::reply::Response;

use futures::stream::StreamExt;

use image::imageops::FilterType;
use image::imageops;
use image::GenericImageView;
use image::ImageFormat;
use image::{DynamicImage, ImageBuffer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResizeMethod {
    Fill,
    Fit,
    Limit,
    Pad
}

impl TryFrom<String> for ResizeMethod {
    type Error = &'static str;


    fn try_from(input: String) -> Result<Self, Self::Error> {
        match input.to_lowercase().as_str() {
            "fill" => Ok(ResizeMethod::Fill),
            "fit" => Ok(ResizeMethod::Fit),
            "limit" => Ok(ResizeMethod::Limit),
            "pad" => Ok(ResizeMethod::Pad),
            _ => Err("invalid resize format")
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ResizeOptions {
    method: Option<ResizeMethod>,
    x: u32,
    y: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let resize_multipart = warp::post()
        .and(warp::path("resize_multipart"))
        .and(form())
        .and(warp::query::<ResizeOptions>())
        .then(resize_multipart_endpoint);

        
    let resize_binary = warp::post()
        .and(warp::path("resize_binary"))
        .and(warp::header::<String>("content-type"))
        .and(warp::filters::body::aggregate())
        .and(warp::query::<ResizeOptions>())
        .then(resize_binary_endpoint);

    let routes = resize_multipart.or(resize_binary);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}


async fn resize_binary_endpoint(content_type: String, data: impl Buf, opts: ResizeOptions) -> Response {
    dbg!(&content_type);

    match inner_resize(data, &content_type, opts) {
        Ok(out) => with_header(out, CONTENT_TYPE, content_type).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn resize_multipart_endpoint(mut form: FormData, opts: ResizeOptions) -> Response {
    let mut file = None;
    let mut filename = None;
    let mut content_type = None;

    while let Some(Ok(mut part)) = form.next().await {
        if let Some(ct) = part.content_type() {
            content_type = Some(ct.to_string());
        }
        if let Some(f) = part.filename() {
            filename = Some(f.to_string());
        }
        if let Some(d) = part.data().await {
            file = Some(d);
        }
    }

    if filename.is_none() {
        return "filename is missing".into_response();
    }
    if content_type.is_none() {
        return "content type is missing".into_response();
    }
    if file.is_none() {
        return "file data is missing".into_response();
    }

    let file = file.unwrap();
    let filename = filename.unwrap();
    let content_type = content_type.unwrap();

    if let Ok(body) = file {
        match inner_resize(body, &content_type, opts) {
            Ok(out) => with_header(out, CONTENT_TYPE, content_type).into_response(),
            Err(e) => e.into_response(),
        }
    } else {
        "error reading body".into_response()
    }
}

fn inner_resize(body: impl Buf, content_type: &str, opts: ResizeOptions) -> Result<Vec<u8>, String> {
    let format = image::ImageFormat::from_mime_type(&content_type)
        .ok_or_else(|| String::from("invalid image format"))?;
    let mut image_data = Vec::new();
    body.reader().read_to_end(&mut image_data).unwrap();
    let image_reader =
        image::load_from_memory_with_format(&image_data, format).map_err(|e| e.to_string())?;

    let method = opts.method.unwrap_or(ResizeMethod::Fit);
    let out_img = resize(&image_reader, format, opts.x, opts.y, method)?;

    Ok(out_img)
}


const PAD_PIXEL: [u8; 3] = [u8::MAX; 3];

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

            let mut out_image = if image.color().has_alpha() {
                image::DynamicImage::new_rgba8(width, height)
            } else {
                // create white image
                DynamicImage::ImageRgb8(ImageBuffer::from_pixel(
                    width,
                    height,
                    image::Rgb::from(PAD_PIXEL),
                ))
            };

            imageops::replace(
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
