use std::cell::Cell;
use std::cell::RefCell;
fn main() {
   println!("RefCell");
   let refcell = RefCell::new(vec![0, 10, 2]);
   let mut mutref = refcell.borrow_mut();
   mutref[0] = 500;
   println!("{:?}", mutref);

   let cell = Cell::new(500);
   let mut v = cell.get();
   v = 50;
   println!("{:?}", cell);
   cell.set(200);
   println!("{:?}", cell);
}