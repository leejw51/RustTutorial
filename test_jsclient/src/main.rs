use std::time;
use jsonrpc_client_transports::transports::ws::connect;
use jsonrpc_client_transports::RawClient;
use jsonrpc_client_transports::RpcChannel;
use jsonrpc_core::Params;
use std::thread;
use tokio::runtime::Runtime;
use websocket:: OwnedMessage;
use serde_json::map::Map;
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
    //map.insert("name".to_string(), "Jeffry".into());
    loop {
        let mut map = Map::new();
        let fut = b.0.call_method("status", Params::Map(map));
        match rt.block_on(fut) {
			Ok(val) => println!("{}",serde_json::to_string_pretty(&val).unwrap()),
			Err(err) => println!("err {:?}",err),
		}
        thread::sleep(time::Duration::from_millis(1000));
    }
}
