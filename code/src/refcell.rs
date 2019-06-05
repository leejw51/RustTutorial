
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Cell;
fn main() {
  let data = Rc::new(RefCell::new(Vec::new()));
  for i in 1..20 {
    println!("{}", i);
    data.borrow_mut().push(i*20);
  }
  println!("{:?}", data);
  println!("{:?}", data.borrow().last());
}