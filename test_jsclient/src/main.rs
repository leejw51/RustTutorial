use std::time;
//use jsonrpc_ws_client::connect;
use futures::future::lazy;
use jsonrpc_client_transports::transports::ws::connect;
use jsonrpc_client_transports::RawClient;
use jsonrpc_client_transports::{RpcChannel, RpcError};
use jsonrpc_core::{Error, ErrorCode, IoHandler, Params, Value};
use std::thread;
use tokio::runtime::Runtime;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};
use serde_json::map::Map;
const CONNECTION: &'static str = "ws://localhost:26657/websocket";
#[derive(Clone)]
struct TestClient(RawClient);

impl From<RpcChannel> for TestClient {
    fn from(channel: RpcChannel) -> Self {
        TestClient(channel.into())
    }
}

fn main() {
    let mut rt = Runtime::new().unwrap();
    let a = connect::<TestClient>(CONNECTION);
    let b: TestClient = rt.block_on(a.unwrap()).unwrap();
    let mut map = Map::new();
    //map.insert("name".to_string(), "Jeffry".into());
    let fut = b.0.call_method("status", Params::Map(map));
	match rt.block_on(fut) {
			Ok(val) => println!("{}",serde_json::to_string_pretty(&val).unwrap()),
			Err(err) => println!("err {:?}",err),
		}

    loop {
        thread::sleep(time::Duration::from_millis(1000));
    }
}
