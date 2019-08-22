extern crate futures;
extern crate tokio;
extern crate websocket;

use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use futures::sync::mpsc::Sender;
//use jsonrpc_ws_client::connect;
use jsonrpc_client_transports::transports::ws::connect;
use jsonrpc_client_transports::{RawClient, TypedClient};
use jsonrpc_client_transports::{RpcChannel, RpcError};
use std::io::stdin;
use std::thread;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};
const CONNECTION: &'static str = "ws://localhost:26657/websocket";
#[derive(Clone)]
struct TestClient(TypedClient);

impl From<RpcChannel> for TestClient {
    fn from(channel: RpcChannel) -> Self {
        TestClient(channel.into())
    }
}

impl TestClient {
    fn hello(&self, msg: &'static str) -> impl Future<Item = String, Error = RpcError> {
        self.0.call_method("hello", "String", (msg,))
    }
    fn fail(&self) -> impl Future<Item = (), Error = RpcError> {
        self.0.call_method("fail", "()", ())
    }
}
fn main() {
    let client = connect::<TestClient>(CONNECTION)
        .and_then(|stream| {
            println!("created stream");

            Ok(())
        })
        .map_err(|err| {
            println!("connection error = {:?}", err);
        });

    println!("OK");
}
