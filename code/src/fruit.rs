pub enum Fruit<T, S> {
    Apple(T),
    Pear(S),
    Strawberry(S),
}
fn main() {
    let m: Fruit<i32, i32> = Fruit::Apple(300);
    let n: Fruit<i32, i32> = Fruit::Pear(200);
    match m {
        Fruit::Apple(tron) => println!("사과  {}", tron),
        Fruit::Pear(star) => println!("배 {}", star),
        Fruit::Strawberry(sea) => println!("딸기 {}", sea),
    }
}