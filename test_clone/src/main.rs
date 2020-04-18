trait Fruit: Send + Sync + Clone {
    fn eat(&self);
}

#[derive(Clone, Default, Debug)]
struct Apple {
    price: i32,
}

impl Apple {}

impl Fruit for Apple {
    fn eat(&self) {
        println!("eat apple {}", self.price);
    }
}

struct Store<T>
where
    T: Fruit,
{
    fruit: T,
}

impl<T> Store<T>
where
    T: Fruit,
{
    fn eat(&self) {
        self.fruit.eat();
    }
}

fn main() {
    let apple = Apple { price: 100 };
    let store = Store {
        fruit: apple.clone(),
    };
    store.eat();
}
