use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::output::GetObjectOutput;
use aws_sdk_s3::Client;
use aws_sdk_s3::Region;
use aws_sdk_s3::{Credentials, Endpoint};
use config::ResizerConfig;
use derivative::Derivative;
use image::imageops::FilterType;
use once_cell::sync::Lazy;
use proto::resize_reply::Status as ReplyStatus;
use proto::resizer_server::{Resizer, ResizerServer};
use proto::{ResizeReply, ResizeRequest};
use resizer::MyResizer;
use s3_config::S3Config;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tonic::{transport::Server, Request, Response, Status};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub mod proto {
    tonic::include_proto!("resizer");
}
pub mod config;
pub mod resizer;
pub mod s3_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ResizerConfig::init();

    let addr = ResizerConfig::address()?;

    let resizer = MyResizer::default();

    println!("ResizerServer listening on {}", addr);

    Server::builder()
        .add_service(ResizerServer::new(resizer))
        .serve(addr)
        .await?;

    Ok(())
}
