use std::time;
//use jsonrpc_ws_client::connect;
use futures::future::lazy;
use jsonrpc_client_transports::transports::ws::connect;
use jsonrpc_client_transports::{RawClient};
use jsonrpc_client_transports::{RpcChannel, RpcError};
use jsonrpc_core::{Error, ErrorCode, IoHandler, Params, Value};
use std::thread;
use tokio::runtime::Runtime;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};
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
    loop {
        thread::sleep(time::Duration::from_millis(1000));
    }
}
