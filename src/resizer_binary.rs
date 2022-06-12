use crate::proto::resize_binary_reply::Status as ReplyStatus;
use crate::proto::resizer_binary_server::ResizerBinary;
use crate::proto::{ResizeBinaryReply, ResizeBinaryRequest};
use crate::config::ResizerConfig;
use tonic::{Request, Response, Status};

fn request_error(error: String) -> Response<ResizeBinaryReply> {
    Response::new(ResizeBinaryReply {
        message: error,
        status: ReplyStatus::Error as i32,
        image: Vec::new(),
    })
}

fn validate_request(request: &ResizeBinaryRequest) -> Result<(), String> {
    match request {
        ResizeBinaryRequest {
            image,
            height,
            width,
            ..
        } if image.is_empty()
            || height == &0
            || width == &0
            || height > &ResizerConfig::max_size()
            || width > &ResizerConfig::max_size() =>
        {
            Err(String::from("invalid arguments"))
        }
        _ => Ok(()),
    }
}


#[derive(Default)]
pub struct ResizerBinaryService;

#[tonic::async_trait]
impl ResizerBinary for ResizerBinaryService {
    async fn resize_binary(
        &self,
        request: Request<ResizeBinaryRequest>,
    ) -> Result<Response<ResizeBinaryReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());


        match inner_resize(request.get_ref()) {
            Ok(data) => Ok(Response::new(ResizeBinaryReply {
                message: String::from("success"),
                status: ReplyStatus::Ok as i32,
                image: data,
            })),
            Err(error) => Ok(request_error(error)),
        }
    }
}

fn inner_resize(request: &ResizeBinaryRequest) -> Result<Vec<u8>, String> {
    validate_request(request)?;

    let image_fmt = image::ImageFormat::from_mime_type(&request.format)
        .ok_or_else(|| String::from("invalid image type"))?;

    let image = image::load_from_memory_with_format(&request.image, image_fmt)
        .map_err(|e| e.to_string())?;
    crate::image::resize(&image, image_fmt, request.width, request.height)
}
