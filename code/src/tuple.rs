#[derive(Debug)]
struct Fruit(String, String, i64, i64);
fn test1() {
    let a = (1, "apple", 100, 20, 30, 50, 80, 90, "pear", "abc", 1, 2000);
    println!("{:?}", a.5);
    println!("OK2");
}
fn main() {
    let a = Fruit("apple".to_string(), "pear".to_string(), 30, 2);
    println!("{:?}", a);
}
