#![deny(clippy::all)]
#![deny(clippy::perf)]
#![deny(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]

use tokio::signal;

use config::ResizerConfig;
use proto::resizer_binary_server::ResizerBinaryServer;
use proto::resizer_server::ResizerServer;
use resizer::ResizerService;
use resizer_binary::ResizerBinaryService;
use s3_config::S3Config;

use tonic::transport::Server;

pub mod proto {
    #![allow(clippy::pedantic)]
    tonic::include_proto!("resizer");
}


pub mod config;
pub mod image;
pub mod resizer;
pub mod resizer_binary;
pub mod s3_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ResizerConfig::init();

    let addr = ResizerConfig::address()?;
    let binary_addr = ResizerConfig::binary_address()?;

    let resizer = ResizerService::default();
    let binary_resizer = ResizerBinaryService::default();

    println!("ResizerServer listening on {}", addr);
    println!("ResizerBinaryServer listening on {}", binary_addr);

    // Server::builder()
    //     .add_service(ResizerServer::new(resizer))
    //     .serve(addr)
    //     .await?;

    tokio::select! {
        _ = signal::ctrl_c() => {},
        _ = Server::builder()
        .add_service(ResizerServer::new(resizer))
        .serve(addr) => {},
        _ = Server::builder()
        .add_service(ResizerBinaryServer::new(binary_resizer))
        .serve(binary_addr) => {},
    }

    Ok(())
}
