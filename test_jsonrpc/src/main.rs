use jsonrpc_core::*;
use jsonrpc_http_server::*;

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
mod worker;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use worker::Worker;
use worker::WorkerShared;
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

    #[rpc(name = "start_work")]
    fn start_work(&self, name: String) -> Result<String>;
}

pub struct App {
    worker: WorkerShared,
}

impl AppInterface for App {
    fn get_number(&self, name: String) -> Result<String> {
        Ok(format!("apple={}", name))
    }
    fn get_fruit(&self, my: My) -> Result<String> {
        Ok(format!("{}={}", my.name, my.price))
    }
    fn start_work(&self, name: String) -> Result<String> {
        if self.worker.lock().unwrap().exist(&name) {
            return Ok("FAIL ALREADY EXISTS".to_string());
        }
        let m = format!("Started {}", name);
        let worker = self.worker.clone();
        let child = thread::spawn(move || {
            let tmpworker = worker;
            tmpworker.lock().unwrap().add(&name);
            for i in 0..20 {
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("waiting {}", i);
            }
            tmpworker.lock().unwrap().remove(&name);
        });

        //    let res = child.join();
        Ok(m)
    }
}

impl App {
    pub fn new() -> Self {
        App {
            worker: Arc::new(Mutex::new(Worker::new())),
        }
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
