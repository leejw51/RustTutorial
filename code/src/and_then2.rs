
fn main() {
  let a: Result<i32, i32> = Ok(2);
  let b = a.map(|e| 5).and_then(|e| Ok(e + 5)).and_then(|e| Ok(e + 5));

  println!("{:?}", b);
}
