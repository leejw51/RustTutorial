
trait Food {
    fn is_food(&self) -> bool;
}
trait Vegetable {
    fn is_vegetable(&self) -> bool;
}
trait Fruit: Food + Vegetable {
    fn eat2(&self);
}

struct Apple {
    name: String,
    price: i64,
}

impl Fruit for Apple {
    fn eat2(&self) {
        println!("eat!");
    }
}

impl Vegetable for Apple {
    fn is_vegetable(&self) -> bool {
        return true;
    }
}

impl Food for Apple {
    fn is_food(&self) -> bool {
        return true;
    }
}
fn main() {
    let a = Apple {
        name: "pineapple".to_string(),
        price: 200,
    };
    a.eat2();
    println!("is_food {}", a.is_food());
    println!("is_vegetable {}", a.is_vegetable());
}