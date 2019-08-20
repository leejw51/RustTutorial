use std::rc::Rc;
fn main() {
   let a = Rc::new("apple".to_string());
   let b = a.clone();
   println!("{:?}", a);
   println!("{:?}", *a);
   println!("{:?}", &a);
    println!("{:?}", b);
}