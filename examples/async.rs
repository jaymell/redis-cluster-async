use futures::stream::StreamExt;
use redis::{AsyncCommands, AsyncIter};

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    println!("HERE1");
    let client = redis_cluster_async::Client::open(vec!["redis://127.0.0.1:6380/"]).unwrap();
    println!("HERE2");
    let mut con = client.get_connection().await?;

    println!("HERE3");
    con.set("async-key1", b"foo").await?;
    // con.incr("async-key1", 1).await?;

    println!("HERE4");
    loop {
        match con.get::<&str, String>("async-key1").await {
            Ok(g) => assert_eq!(g, "foo"),
            Err(e) => println!("{}", e.to_string()),
        }
    }

    println!("HERE5");
    Ok(())
}

