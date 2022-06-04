use crate::proto::resize_reply::Status as ReplyStatus;
use crate::proto::resizer_server::{Resizer, ResizerServer};
use crate::proto::{ResizeReply, ResizeRequest};
use crate::ResizerConfig;
use crate::S3Config;
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::output::GetObjectOutput;
use aws_sdk_s3::Client;
use aws_sdk_s3::Region;
use aws_sdk_s3::{Credentials, Endpoint};
use derivative::Derivative;
use image::imageops::FilterType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tonic::{transport::Server, Request, Response, Status};
use zeroize::{Zeroize, ZeroizeOnDrop};

fn request_error(error: String) -> Result<Response<ResizeReply>, Status> {
    Ok(Response::new(ResizeReply {
        message: error,
        status: ReplyStatus::Error as i32,
    }))
}

#[derive(Default)]
pub struct MyResizer {}

#[tonic::async_trait]
impl Resizer for MyResizer {
    async fn resize(
        &self,
        request: Request<ResizeRequest>,
    ) -> Result<Response<ResizeReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let inner_request = match validate_request(request) {
            Ok(inner_request) => inner_request,
            Err(error) => return request_error(error),
        };

        let config = match decrypt_config(&inner_request.config) {
            Ok(config) => config,
            Err(error) => return request_error(error),
        };

        let client = config.to_client().await;

        match Self::inner_resize(client, inner_request).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(error) => request_error(error),
        }
    }
}

fn validate_request(request: Request<ResizeRequest>) -> Result<ResizeRequest, String> {
    match request.into_inner() {
        ResizeRequest {
            input,
            output,
            bucket,
            height,
            width,
            ..
        } if bucket == ""
            || input == ""
            || output == ""
            || height == 0
            || width == 0
            || height > ResizerConfig::max_size()
            || width > ResizerConfig::max_size() =>
        {
            Err(String::from("invalid arguments"))
        }
        ResizeRequest { config, .. } if config.len() == 0 => Err(String::from("missing config")),
        other => Ok(other),
    }
}

fn decrypt_config(data: &[u8]) -> Result<S3Config, String> {
    if data.len() < 12 {
        return Err(String::from("invalid config"));
    };

    let (iv, bytes) = data.split_at(12);
    let nonce = Nonce::from_slice(iv);

    let cipher = Aes256Gcm::new(&ResizerConfig::shared_key());
    let plaintext = cipher.decrypt(nonce, bytes).map_err(|e| e.to_string())?;

    let s3_config: S3Config = serde_json::from_slice(&plaintext).map_err(|e| e.to_string())?;
    Ok(s3_config)
}

impl MyResizer {
    async fn inner_resize(client: Client, request: ResizeRequest) -> Result<ResizeReply, String> {
        if let Ok(GetObjectOutput {
            body,
            content_type: Some(content_type),
            ..
        }) = client
            .get_object()
            .bucket(&request.bucket)
            .key(request.input)
            .send()
            .await
        {
            let image_fmt = image::ImageFormat::from_mime_type(&content_type)
                .ok_or(String::from("invalid image type"))?;
            let image = body
                .collect()
                .await
                .map(|data| data.into_bytes())
                .map_err(|e| e.to_string())
                .and_then(|data| {
                    image::load_from_memory_with_format(&data, image_fmt).map_err(|e| e.to_string())
                })?;

            let filter_type = if needs_upscale(&image, request.width, request.height) {
                FilterType::CatmullRom
            } else {
                FilterType::Lanczos3
            };

            let new_image = image.resize(request.width, request.height, filter_type);
            // let new_image = image.resize_exact(request.width, request.height, filter_type);

            let mut bytes: Vec<u8> = Vec::new();
            new_image
                .write_to(&mut Cursor::new(&mut bytes), image_fmt)
                .map_err(|e| e.to_string())?;

            client
                .put_object()
                .bucket(&request.bucket)
                .key(request.output)
                .body(bytes.into())
                .content_type(content_type)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            Ok(ResizeReply {
                message: format!("Works! with content type {:?}", image_fmt),
                status: ReplyStatus::Ok as i32,
            })
        } else {
            Err(String::from("invalid file type"))
        }
    }
}

fn needs_upscale(image: &image::DynamicImage, width: u32, height: u32) -> bool {
    if image.width() >= width {
        return true;
    };
    if image.height() >= height {
        return true;
    };
    false
}
