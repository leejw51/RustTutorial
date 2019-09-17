use sled::Db;

fn run() -> Result<(), ()> {
    let path = ".db";
    let _tree = Db::open(path).map_err(|_e| {})?;
    Ok(())
}
fn main() {
    run().expect("OK");
    println!("Hello, world!");
}
