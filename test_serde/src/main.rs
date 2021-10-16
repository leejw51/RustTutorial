use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Default,PartialEq,Debug)]
struct Point {
  x:i32,
  y:i32,
  name:String,
}

fn main() {
  let point = Point {x:10,y:20, name:"apple".to_string()};
  let m = serde_json::to_string(&point).unwrap();
  let m2: Point = serde_json::from_str(&m).unwrap();
  let m3 = serde_json::to_string(&m2).unwrap();
  println!("{}", m);
  println!("{:?}", m2);
  println!("{}", m3);

  let encoded: Vec<u8> = bincode::serialize(&point).unwrap();
  println!("encode={}",hex::encode(&encoded));
  let decoded: Point = bincode::deserialize(&encoded).unwrap();
  println!("decode={:?}", decoded);
}
