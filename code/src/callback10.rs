pub type MyCallback=fn(i32) -> i32;

struct Network {
    count: i32, 
    name: &'static str, 
    onRead:MyCallback,
    onWrite:MyCallback, 
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
      println!("value2={}",a);
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