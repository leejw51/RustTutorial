
use std::error::Error;
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::io::AsyncReadExt;
use crossbeam::channel;

async fn async_get_name() -> Result<String, Box<dyn Error + Send + Sync>> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok("OK".into())
}

fn sync_get_name(handle: Handle) -> Result<String, Box<dyn Error + Send + Sync>> {
    let (tx, rx) = channel::bounded(1);
    handle.spawn(async move {
        let score_res = async_get_name().await;
        let _ = tx.send(score_res);
    });
    Ok(rx.recv()??)
}

#[tokio::main]
 async fn main() {
    println!("Hello world");
    let res = tokio::task::spawn_blocking(|| {
        let score = sync_get_name(tokio::runtime::Handle::current()).unwrap();
        // print score
        println!("score: {}", score);
    }).await.unwrap();
    // call get_score_sync
  
}
