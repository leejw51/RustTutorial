use futures::future::Future;
use futures::stream::Stream;
use jsonrpc_client_transports::transports::ws::connect;
use jsonrpc_client_transports::RawClient;
use jsonrpc_client_transports::RpcChannel;
use jsonrpc_core::types::params::Params;
use serde_json::map::Map;
use std::thread;
use std::time;
use tokio::runtime::Runtime;
use websocket::OwnedMessage;

use tokio::io;
use tokio::net::TcpStream;

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
    // let fut = b.0.call_method("subscribe", Params::Map(map));
    //rt.block_on(fut);
    let fut =
        b.0.subscribe("subscribe", Params::Map(map), "subscribe", "");
    let stream = rt.block_on(fut).unwrap();
    let fut2 = stream.into_future();
    let fut3 = fut2.map(|a| {});
    rt.block_on(fut3);
}

fn main2() {
    let addr = "127.0.0.1:26657".parse().unwrap();

    let future = TcpStream::connect(&addr)
        .and_then(|socket| io::write_all(socket, b"status"))
        .and_then(|(socket, _)| {
            println!("read bytes");
            // read exactly 11 bytes
            io::read_exact(socket, vec![0; 20])
        })
        .and_then(|(socket, buf)| {
            println!("got {:?}", buf);
            Ok(())
        })
        .map_err(|_| println!("failed"));

    tokio::run(future);
}
