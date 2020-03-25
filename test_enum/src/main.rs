
#[derive(Debug)]
enum Fruit {
    Apple=5,
    Pear,
    Strawberry,
}

impl From<u64> for Fruit {
    fn from(code:u64) -> Self {
        match code {
            5=> Fruit::Apple,
            6=> Fruit::Pear,
            _=> Fruit::Strawberry,
        }
    }
}

fn main() {
    let m:u64= Fruit::Pear as u64;
    println!("{}",m);
    let n: Fruit= m.into();
    println!("{:?}",n);
}
