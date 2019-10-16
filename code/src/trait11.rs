struct Fruit {}

trait Food {
    fn eat(&self) {
        println!("eat food");
    }
}

impl Food for Fruit {
    fn eat(&self) {
        println!("eat fruit");
    }
}
fn main() {
    let a = Fruit {};
    a.eat();
    println!("fruit");
}
