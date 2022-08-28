use futures::prelude::*;
use redis::AsyncCommands;
use redis::{RedisError, ErrorKind, RedisResult, Value};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Message {
    image_key: String,
    resize_key: String,
}

impl redis::FromRedisValue for Message {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let out: String = redis::FromRedisValue::from_redis_value(v)?;

        if let Some((image_key, resize_key)) = out.split_once(';') {
            return Ok(Message{image_key: image_key.to_string(), resize_key: resize_key.to_string()})
        };

        Err(RedisError::from((
            ErrorKind::TypeError,
            "Response was of incompatible type",
            format!("{:?} (response was {:?})", "No image and resize key found", v),
        )))
    }
}

#[derive(Debug, PartialEq)]
struct ResizeOption {
    x: u32,
    y: u32,
    resize_method: String
}

impl redis::FromRedisValue for ResizeOption {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let out: String = redis::FromRedisValue::from_redis_value(v)?;

        if let Some((dimention, resize_method)) = out.split_once('$') {
            if let Some((x, y)) = dimention.split_once('x') {
                let x: u32 = redis::FromRedisValue::from_redis_value(&Value::Status(x.to_string()))?;
                let y: u32 = redis::FromRedisValue::from_redis_value(&Value::Status(y.to_string()))?;

                return Ok(ResizeOption{x, y, resize_method: resize_method.to_string()})
            }
        };

        Err(RedisError::from((
            ErrorKind::TypeError,
            "Response was of incompatible type",
            format!("{:?} (response was {:?})", "No resize option found", v),
        )))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_tokio_connection_manager().await?;
    let mut pubsub_conn = client.get_async_connection().await?.into_pubsub();

    let data: RedisResult<Value> = conn.get("hallo").await;

    dbg!(data);

    pubsub_conn.subscribe("channel_1").await?;
    let mut pubsub_stream = pubsub_conn.on_message();

    while let Some(msg) = pubsub_stream.next().await {
        if let Ok(data) = msg.get_payload::<Message>() {
            let image_binary: RedisResult<Vec<u8>> = conn.get(&data.image_key).await;
            let resize_options: RedisResult<HashMap<String, ResizeOption>> = conn.hgetall(&data.resize_key).await;

            dbg!(resize_options);
        }
    }

    Ok(())
}
