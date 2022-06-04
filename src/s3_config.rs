use crate::proto::resize_reply::Status as ReplyStatus;
use crate::proto::resizer_server::{Resizer, ResizerServer};
use crate::proto::{ResizeReply, ResizeRequest};
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

#[derive(Debug, Derivative, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
#[derivative(Default)]
pub struct S3Config {
    #[derivative(Default(value = "\"us-east-1\".to_string()"))]
    region: String,
    #[derivative(Default(value = "\"http://127.0.0.1:9000\".to_string()"))]
    endpoint: String,
    #[derivative(Default(value = "\"test_key_id\".to_string()"))]
    access_key: String,
    #[derivative(Default(value = "\"secret_access_key\".to_string()"))]
    secret_access_key: String,
}

impl S3Config {
    fn endpoint(&self) -> http::Uri {
        if let Ok(endpoint) = self.endpoint.parse() {
            endpoint
        } else {
            "http://localhost:9000".parse().expect("valid URI")
        }
    }

    pub async fn to_client(self) -> Client {
        let region_provider = RegionProviderChain::first_try(Region::new(self.region.to_string()))
            .or_else("us-east-1");
        let creds = Credentials::new(&self.access_key, &self.secret_access_key, None, None, "s3");
        let endpoint = Endpoint::immutable(self.endpoint());

        let config = aws_config::from_env()
            .region(region_provider)
            .credentials_provider(creds)
            .endpoint_resolver(endpoint)
            .load()
            .await;
        Client::new(&config)
    }
}
