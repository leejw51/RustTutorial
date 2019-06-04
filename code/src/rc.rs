use std::rc::Rc;
fn main() {
   let a = Rc::new("apple".to_string());
   println!("{:?}", a);
   println!("{:?}", *a);
   println!("{:?}", &a);
}