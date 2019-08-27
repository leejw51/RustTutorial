use futures::future::lazy;
use futures::sync::oneshot;
use futures::Future;
use jsonrpc_client_transports::transports::ws::connect;
use jsonrpc_client_transports::RawClient;
use jsonrpc_client_transports::RpcChannel;
use jsonrpc_core::types::params::Params;
use serde_json::map::Map;
use std::thread;
use std::time;
use tokio::runtime::Runtime;
use websocket::OwnedMessage;
const CONNECTION: &'static str = "ws://localhost:26657/websocket";
#[derive(Clone)]
struct MyClient(RawClient);

impl From<RpcChannel> for MyClient {
    fn from(channel: RpcChannel) -> Self {
        MyClient(channel.into())
    }
}

fn main() {
    let mut rt = Runtime::new().unwrap();
    let a = connect::<MyClient>(CONNECTION);
    let b: MyClient = rt.block_on(a.unwrap()).unwrap();
    println!("connected..");

    let mut map = Map::new();
    map.insert("query".to_string(), "tm.event='NewBlock'".into());
    let fut = b.0.subscribe("subscribe", Params::Map(map), "", "");
    let stream=rt.block_on(fut).unwrap();
    println!("subscribed ok!");
    loop {
        thread::sleep(time::Duration::from_millis(1000));
    }
    println!("done");
}
