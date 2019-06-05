use std::sync::Arc;
use std::rc::Rc;

fn test1() {
   let mut x = Arc::new(3);
   *Arc::get_mut(&mut x).unwrap() = 4;
   println!("x={}", x);
   let _y = Arc::clone(&x);
   let _z = _y.clone();
   println!("_y={}", _y);
   println!("_z={}", _z);
}

fn main() {
  let mut x = Rc::new(3);
   *Rc::get_mut(&mut x).unwrap() = 4;
   println!("x={}", x);
   let _y = Rc::clone(&x);
   let _z = _y.clone();
   println!("_y={}", _y);
   println!("_z={}", _z);

}
