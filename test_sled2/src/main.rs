mod db;
mod fruit;
mod drink;
fn main() {
    db::main();
    println!("Hello, world!");
    fruit::apple::main();
}
