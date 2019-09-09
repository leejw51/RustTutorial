mod client;
mod server;

use quest::{ask, text, yesno};
use crate::server::server;
use crate::client::client;

fn main() {
    loop {
        ask("1. server\n");
        ask("2. client\n");
        ask("q. exit\n");
        let a = text().unwrap();
        match a.as_str() {
            "1" => {
                server();
            }
            "2" => {
                client();
            }
            "q" => {
                break;
            }
            _ => {}
        }
    }
}
