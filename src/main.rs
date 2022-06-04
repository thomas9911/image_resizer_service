#![deny(clippy::all)]
#![deny(clippy::perf)]
#![deny(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::similar_names)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]

use config::ResizerConfig;
use proto::resizer_server::ResizerServer;
use resizer::ResizerService;
use s3_config::S3Config;

use tonic::transport::Server;

pub mod proto {
    #![allow(clippy::pedantic)]
    tonic::include_proto!("resizer");
}
pub mod config;
pub mod resizer;
pub mod s3_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ResizerConfig::init();

    let addr = ResizerConfig::address()?;

    let resizer = ResizerService::default();

    println!("ResizerServer listening on {}", addr);

    Server::builder()
        .add_service(ResizerServer::new(resizer))
        .serve(addr)
        .await?;

    Ok(())
}
