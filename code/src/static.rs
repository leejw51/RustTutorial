#[macro_use]
extern crate lazy_static;

use std::env;

lazy_static! {
    static ref EXAMPLE: u8 = {
        let args: Vec<String> = env::args().collect();
        let ret: u8 = args[1].parse().unwrap();
        ret
    };
}
fn main() {
    println!("runtime option={}", *EXAMPLE);
}
