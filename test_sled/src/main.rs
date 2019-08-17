use sled::Db;
use std::str;
fn main() {
    let tree = Db::open("./disk").unwrap();

    // set and get
    tree.insert(b"ok", b"apple".to_vec());
    tree.insert(b"ok", b"apple5".to_vec());
    println!("{:?}", str::from_utf8(&tree.get(b"ok").unwrap_or_default().unwrap_or_default()).unwrap_or_default());
    match tree.get(b"ok") {
        Ok(a) => {
            match a {
                Some(c) => {
                    let r= str::from_utf8(&c).unwrap();
                    println!("{:?}",r);

                },
                _=>{}

            }
        },
        _ => {},
    }
        // block until all operations are on-disk
    tree.flush();
}
