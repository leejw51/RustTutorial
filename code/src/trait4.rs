trait Item {
     fn get_price(&self) ->i64 {
        return 0;
    }
}

struct Sword {

}

impl Item for Sword {
    fn get_price(&self)->i64 {
        10000
    }
}
fn main() {
    let a= Sword{};
    let c: &Item= & a;
    let b= c.get_price();
    println!("price={}",b);
}