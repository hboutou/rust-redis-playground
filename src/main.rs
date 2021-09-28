#![allow(unused)]

use std::{error::Error, time::Duration};
use tokio::time::sleep;

use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // 1) create connection
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_tokio_connection().await?;

    // 2) set / get key
    con.set("my-key", "hello-redis").await?;
    let result: String = con.get("my-key").await?;
    println!("my-key: {}", result);

    // 3) xadd to redis streams
    con.xadd(
        "my-stream",
        "*",
        &[("firstname", "hamza"), ("lastname", "ait boutou")]
    ).await?;
    let len: i32 = con.xlen("my-stream").await?;
    println!("len of my-stream: {}", len);

    // 4) xrevrange the read stream
    let result : Option<StreamRangeReply> = con.xrevrange_count(
        "my-stream",
        "+",
        "-",
        10
    ).await?;

    if let Some(reply) = result {
        for stream_id in reply.ids {
            println!("xrevrang stream message {}", stream_id.id);
            for (name, value) in stream_id.map.iter() {
                println!("{}: {}", name, from_redis_value::<String>(value)?);
            }
            println!();
        }
    }


    // Final wait and clean up
    con.del("my-key").await?;
    con.del("my-stream").await?;
    Ok(())
}
