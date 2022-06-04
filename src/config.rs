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

static CONFIG: Lazy<ResizerConfig> = Lazy::new(|| {
    let mut config = ResizerConfig::default();
    config.from_env();
    config
});

#[derive(Derivative, Zeroize, ZeroizeOnDrop)]
#[derivative(Default)]
/// wrapper around once_cell
pub struct ResizerConfig {
    #[derivative(Default(value = "\"[::1]:50051\".to_string()"))]
    address: String,
    #[derivative(Default(value = "4000"))]
    max_size: u32,
    shared_key: Option<String>,
}

impl ResizerConfig {
    pub fn init() {
        Lazy::force(&CONFIG);
        Self::shared_key();
    }

    pub fn from_env(&mut self) {
        if let Ok(var) = std::env::var("IMAGE_RESIZER_ADDRESS") {
            self.address = var;
        };
        if let Ok(var) = std::env::var("IMAGE_RESIZER_MAX_SIZE") {
            if let Ok(var) = var.parse() {
                self.max_size = var;
            }
        };

        if let Ok(var) = std::env::var("IMAGE_RESIZER_SHARED_KEY") {
            self.shared_key = Some(var);
        };
    }

    pub fn address(
    ) -> Result<std::net::SocketAddr, <std::net::SocketAddr as std::str::FromStr>::Err> {
        CONFIG.address.parse()
    }

    pub fn max_size() -> u32 {
        CONFIG.max_size
    }

    pub fn shared_key() -> Key<<Aes256Gcm as NewAead>::KeySize> {
        let key = CONFIG.shared_key.as_ref().expect("shared key not set");
        let key = base64::decode(key).expect("shared key invalid base64");
        if key.len() != 32 {
            panic!("shared key invalid length")
        };
        Key::from_slice(&key).to_owned()
    }
}
