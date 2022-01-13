
use std::error::Error;
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::io::AsyncReadExt;
use crossbeam::channel;

async fn async_get_name() -> anyhow::Result<String> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok("OK".into())
}

fn sync_get_name(handle: Handle) -> anyhow::Result<String> {
    let (tx, rx) = channel::bounded(1);
    handle.spawn(async move {
        let score_res = async_get_name().await;
        let _ = tx.send(score_res);
    });
    Ok(rx.recv()??)
}


  fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async {
        let score = sync_get_name(rt.handle().clone()).unwrap();
        println!("score: {}", score);
        score
    });
    // get score
    println!("score: {}", res);
}
