use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::output::GetObjectOutput;
use aws_sdk_s3::Client;
use aws_sdk_s3::{Credentials, Endpoint};
use hello_world::resize_reply::Status as ReplyStatus;
use hello_world::resizer_server::{Resizer, ResizerServer};
use hello_world::{ResizeReply, ResizeRequest};
use image::imageops::FilterType;
use std::io::Cursor;
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
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

        let inner_request = if let Ok(inner_request) = validate_request(request) {
            inner_request
        } else {
            return Ok(Response::new(hello_world::ResizeReply {
                message: String::from("invalid arguments"),
                status: ReplyStatus::Error as i32,
            }));
        };

        let client = s3_client().await;

        let reply = match Self::inner_resize(client, inner_request).await {
            Ok(reply) => reply,
            Err(error) => hello_world::ResizeReply {
                message: error,
                status: ReplyStatus::Error as i32,
            },
        };

        Ok(Response::new(reply))
    }
}

fn validate_request(request: Request<ResizeRequest>) -> Result<ResizeRequest, ()> {
    println!("{:?}", request.get_ref());
    match request.into_inner() {
        ResizeRequest {
            input,
            output,
            bucket,
            height,
            width,
        } if bucket == ""
            || input == ""
            || output == ""
            || height == 0
            || width == 0
            || height > 4000
            || width > 4000 =>
        {
            Err(())
        }
        other => Ok(other),
    }
}

impl MyResizer {
    async fn inner_resize(
        client: Client,
        request: ResizeRequest,
    ) -> Result<hello_world::ResizeReply, String> {
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
            let image_fmt = image::ImageFormat::from_mime_type(content_type)
                .ok_or(String::from("invalid image type"))?;
            let image: Result<image::DynamicImage, String> = body
                .collect()
                .await
                .map(|data| data.into_bytes())
                .map_err(|e| e.to_string())
                .and_then(|data| {
                    image::load_from_memory_with_format(&data, image_fmt).map_err(|e| e.to_string())
                });

            let image = image?;

            let filter_type = if is_larger(&image, request.width, request.height) {
                FilterType::CatmullRom
            } else {
                FilterType::Lanczos3
            };

            let new_image = image.resize_exact(request.width, request.height, filter_type);

            let mut bytes: Vec<u8> = Vec::new();
            new_image
                .write_to(&mut Cursor::new(&mut bytes), image_fmt)
                .map_err(|e| e.to_string())?;

            client
                .put_object()
                .bucket(&request.bucket)
                .key(request.output)
                .body(bytes.into())
                .send()
                .await
                .map_err(|e| e.to_string())?;

            Ok(hello_world::ResizeReply {
                message: format!("Works! with content type {image_fmt:?}",),
                status: ReplyStatus::Ok as i32,
            })
        } else {
            Err(String::from("invalid file type"))
        }
    }
}

fn is_larger(image: &image::DynamicImage, width: u32, height: u32) -> bool {
    if image.width() >= width {
        return true
    };
    if image.height() >= height {
        return true
    };
    false
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let resizer = MyResizer::default();

    println!("ResizerServer listening on {}", addr);

    Server::builder()
        .add_service(ResizerServer::new(resizer))
        .serve(addr)
        .await?;

    Ok(())
}

async fn s3_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let creds = Credentials::new("test_key_id", "secret_access_key", None, None, "s3");
    let endpoint = Endpoint::immutable("http://127.0.0.1:9000".parse().expect("valid URI"));

    // let config = aws_config::from_env().region(region_provider).load().await;
    let config = aws_config::from_env()
        .region(region_provider)
        .credentials_provider(creds)
        .endpoint_resolver(endpoint)
        .load()
        .await;
    Client::new(&config)
}
