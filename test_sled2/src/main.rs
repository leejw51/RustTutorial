mod db;
mod fruit;
fn main() {
    db::main();
    println!("Hello, world!");
    fruit::apple::main();
}
