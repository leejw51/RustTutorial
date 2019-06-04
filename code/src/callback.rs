
type Callback = fn();

struct Processor {
   callback: Callback,
}

impl Processor {
   fn set_callback(&mut self, c: Callback) {
      self.callback = c;
   }

   fn process_events(&self) {
      (self.callback)();
   }
}

fn simple_callback() {
   println!("hello world!");
}

fn main() {
   let mut p = Processor {
      callback: simple_callback,
   };
   p.process_events(); // hello world!
}