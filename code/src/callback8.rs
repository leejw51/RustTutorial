
struct Network<T> where T: Fn(i32)->i32 {
  count: i32, 
  name: &'static str, 
  onRead:T,
  onWrite:T, 
}

impl<T> Network<T> where T: Fn(i32)->i32 {

  fn read(&mut self) {
    self.count+=1;
    (self.onRead)(100);
  }

  fn write(&mut self) {
    self.count+=2;
    (self.onWrite)(200);
  }
}
fn main () {
  println!("OK");
  let dummy= |a| {
    println!("value={}",a);
    a
  };
  let mut m = Box::new( Network{
    count:0,
    name:"earth",
    onRead:dummy,
    onWrite:dummy,
  });
  m.read();
  m.write();
}