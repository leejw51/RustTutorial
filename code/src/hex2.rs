
extern crate hex;
use std::str;
fn main() {
    let a = "account";
    let b = a.as_bytes();
    let c = format!("0x{}", hex::encode(a));
    let d = &c[2..];
    let e = hex::decode(d).unwrap();
    let f = str::from_utf8(&e).unwrap();
    println!("{}", c);
    println!("{}", d);
    println!("{}", f);
}