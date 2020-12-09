use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Default,Debug)]
struct Point {
  x:i32,
  y:i32,
  name:String,
}

fn main() {
  let point = Point::default();
  let m = serde_json::to_string(&point).unwrap();
  let m2: Point = serde_json::from_str(&m).unwrap();
  let m3 = serde_json::to_string(&m2).unwrap();
  println!("{}", m);
  println!("{:?}", m2);
  println!("{}", m3);
}
