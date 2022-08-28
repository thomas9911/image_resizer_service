use futures::prelude::*;
use redis::AsyncCommands;
use redis::{RedisResult, Value};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_tokio_connection_manager().await?;
    let mut pubsub_conn = client.get_async_connection().await?.into_pubsub();

    let mut file = File::open("demo.jpg").await?;
    
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;

    conn.set("my_image", contents).await?;
    conn.hset_multiple("my_image_sizes", &[
        ("my_size", "150x150$fill"),
        ("my_size_2", "150x150$fit"),
    ]).await?;

    conn.publish("channel_1", "my_image;my_image_sizes").await?;

    Ok(())
}
