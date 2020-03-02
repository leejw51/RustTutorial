use std::mem;
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Fruit {
    name: String,
    price: i32,
}

fn main() {
    let b = Box::new(Fruit {
        name: "apple".to_string(),
        price: 200,
    });
    let c = b.clone();
     println!("{:?}",b);
     println!("{:?}",c);
}