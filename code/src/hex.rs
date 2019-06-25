extern crate hex;
fn main() {
    let a = "ABCDE2";
    let b= "abcde2";
    println!("{:?}", hex::decode(a));
    println!("{:?}", hex::decode(b));
}