use jsonrpc_core::*;
use jsonrpc_http_server::*;

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct My {
    price: u32,
    name: String,
}
/*
{
	"method": "get_number",
	"jsonrpc": "2.0",
	"params": ["pear"],
	"id": "wallet_restore_hd"
}
*/
#[rpc]
pub trait AppInterface: Send + Sync {
    #[rpc(name = "get_number")]
    fn get_number(&self, name: String) -> Result<String>;

    #[rpc(name = "get_fruit")]
    fn get_fruit(&self, my: My) -> Result<String>;
}

pub struct App {}

impl AppInterface for App {
    fn get_number(&self, name: String) -> Result<String> {
        Ok(format!("apple={}", name))
    }
    fn get_fruit(&self, my: My) -> Result<String> {
        Ok(format!("{}={}",my.name, my.price))
    }
}

impl App {
    pub fn new() -> Self {
        App {}
    }
}

fn main() {
    let m = App::new();
    let mut io = IoHandler::new();
    io.add_method("say_hello", |_: Params| {
        Ok(Value::String("hello".to_string()))
    });

    io.extend_with(m.to_delegate());

    let _server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
        ]))
        .start_http(&"0.0.0.0:3030".parse().unwrap())
        .expect("Unable to start RPC server");

    _server.wait();
}
