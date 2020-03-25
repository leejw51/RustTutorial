use jsonrpc_core::*;
use jsonrpc_http_server::*;

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait AppInterface: Send+ Sync {
    #[rpc(name="get_number")]
    fn get_number(&self, name:String)-> Result<String> ;
}


pub struct App {

}

impl AppInterface for App {
    fn get_number(&self, name:String)-> Result<String> 
    {
        Ok(format!("apple={}", name))
    }

}

impl App {
    pub fn new()->Self {
        App {

        }
    }
}


fn main() {
    let m = App::new();
    let mut io = IoHandler::new();
    io.add_method("say_hello", |_: Params| {
        Ok(Value::String("hello".to_string()))

    });

    io.extend_with((m.to_delegate()));

    let _server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
        ]))
        .start_http(&"0.0.0.0:3030".parse().unwrap())
        .expect("Unable to start RPC server");

    _server.wait();
}
