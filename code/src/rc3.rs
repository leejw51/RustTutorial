use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
struct Node {
    price: i32,
}
impl Node {
    fn print(&self) {
        println!("price={}", self.price);
    }
    fn set(&mut self, newvalue: i32) {
        self.price = newvalue;
    }
}

fn main() {
    let mut n = Rc::new(RefCell::new(Node { price: 20 }));
    let mut m = n.clone();
    n.borrow_mut().set(200);
    n.borrow().print();
    m.borrow().print();
}
