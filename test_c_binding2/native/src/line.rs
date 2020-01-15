#[repr(C)]
pub struct Fruit {
    name: String,
    price: i64,
}


impl Fruit {
    #[no_mangle]
    pub fn price(self) {
        println!("price={}", self.price);
    }
} 


#[no_mangle]
pub fn check() -> i64 {
    0
}
