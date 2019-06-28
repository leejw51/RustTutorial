extern crate hex;
use std::str;
fn main() {
    let a= vec![48,100,98];
    let b= str::from_utf8(&a).unwrap();
    println!("{}", b);
}
