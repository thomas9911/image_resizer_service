use crate::proto::resize_reply::Status as ReplyStatus;
use crate::proto::resizer_server::Resizer;
use crate::proto::{ResizeMethod, ResizeReply, ResizeRequest};
use crate::ResizerConfig;
use crate::S3Config;
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aws_sdk_s3::output::GetObjectOutput;
use aws_sdk_s3::types::AggregatedBytes;
use aws_sdk_s3::Client;
use tonic::{Request, Response, Status};

fn request_error(error: String) -> Response<ResizeReply> {
    Response::new(ResizeReply {
        message: error,
        status: ReplyStatus::Error as i32,
    })
}

#[derive(Default)]
pub struct ResizerService;

#[tonic::async_trait]
impl Resizer for ResizerService {
    async fn resize(
        &self,
        request: Request<ResizeRequest>,
    ) -> Result<Response<ResizeReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let inner_request = match validate_request(request) {
            Ok(inner_request) => inner_request,
            Err(error) => return Ok(request_error(error)),
        };

        let config = match decrypt_config(&inner_request.config, &ResizerConfig::shared_key()) {
            Ok(config) => config,
            Err(error) => return Ok(request_error(error)),
        };

        let client = config.to_client().await;

        match Self::inner_resize(client, inner_request).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(error) => Ok(request_error(error)),
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
        } if bucket.is_empty()
            || input.is_empty()
            || output.is_empty()
            || height == 0
            || width == 0
            || height > ResizerConfig::max_size()
            || width > ResizerConfig::max_size() =>
        {
            Err(String::from("invalid arguments"))
        }
        ResizeRequest { config, .. } if config.is_empty() => Err(String::from("missing config")),
        other => Ok(other),
    }
}

fn decrypt_config(
    data: &[u8],
    key: &Key<<Aes256Gcm as NewAead>::KeySize>,
) -> Result<S3Config, String> {
    if data.len() < 12 {
        return Err(String::from("invalid config"));
    };

    let (iv, bytes) = data.split_at(12);
    let nonce = Nonce::from_slice(iv);

    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher.decrypt(nonce, bytes).map_err(|e| e.to_string())?;

    S3Config::from_json(&plaintext).map_err(|e| e.to_string())
}

impl ResizerService {
    async fn inner_resize(client: Client, request: ResizeRequest) -> Result<ResizeReply, String> {
        let method = ResizeMethod::from_i32(request.method)
            .ok_or_else(|| String::from("invalid resize method"))?;

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
                .ok_or_else(|| String::from("invalid image type"))?;
            let image = body
                .collect()
                .await
                .map(AggregatedBytes::into_bytes)
                .map_err(|e| e.to_string())
                .and_then(|data| {
                    image::load_from_memory_with_format(&data, image_fmt).map_err(|e| e.to_string())
                })?;

            let bytes =
                crate::image::resize(&image, image_fmt, request.width, request.height, method)?;

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

// fn needs_upscale(image: &image::DynamicImage, width: u32, height: u32) -> bool {
//     if image.width() >= width {
//         return true;
//     };
//     if image.height() >= height {
//         return true;
//     };
//     false
// }

#[cfg(test)]
mod decrypt_test {
    use super::*;

    fn setup(my_config: &S3Config, key: &str) -> Vec<u8> {
        // client side
        let plaintext = serde_json::to_vec(&my_config).unwrap();
        let insecure_iv = [1; 12];
        let cipher = Aes256Gcm::new(aes_gcm::Key::from_slice(&base64::decode(key).unwrap()));
        let ciphertext = cipher
            .encrypt(&insecure_iv.into(), plaintext.as_ref())
            .unwrap();

        let mut data = Vec::new();
        data.extend_from_slice(&insecure_iv);
        data.extend_from_slice(&ciphertext);

        data
    }

    #[test]
    fn works() {
        let my_config = S3Config::default();
        let key = ResizerConfig::generate_shared_key();

        let data = setup(&my_config, &key);

        // server side
        let key = *Key::from_slice(&base64::decode(key).unwrap());
        let out_config = decrypt_config(&data, &key).unwrap();

        assert_eq!(my_config, out_config);
    }

    #[test]
    fn invalid_secret_key() {
        let my_config = S3Config::default();
        let in_key = ResizerConfig::generate_shared_key();
        let out_key = ResizerConfig::generate_shared_key();

        let data = setup(&my_config, &in_key);

        // server side
        let key = *Key::from_slice(&base64::decode(out_key).unwrap());
        let out_config = decrypt_config(&data, &key);

        assert_eq!(Err("aead::Error".to_string()), out_config);
    }
}
