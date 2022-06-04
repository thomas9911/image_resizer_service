use config::ResizerConfig;
use proto::resizer_server::ResizerServer;
use resizer::MyResizer;
use s3_config::S3Config;

use tonic::transport::Server;

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
