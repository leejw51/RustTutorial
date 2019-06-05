
struct Network {
  count: i32, 
  name: &'static str, 
  onRead:Box<FnMut(i32)->i32>,
  onWrite:Box<FnMut(i32)->i32>, 
}

impl Network  {

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
    onRead:Box::new(dummy),
    onWrite:Box::new(dummy),
  });
  m.read();
  m.write();
}