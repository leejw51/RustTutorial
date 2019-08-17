use sled::Db;
use std::str;
fn main() {
    let tree = Db::open("./disk").unwrap();

    // set and get
    tree.insert(b"ok", b"apple".to_vec());
    tree.insert(b"ok", b"apple5".to_vec());
    let a= tree.get(b"ok").unwrap().unwrap();
    let b= str::from_utf8(&a);
    println!("{:?}",b);

    // block until all operations are on-disk
    tree.flush();
}
