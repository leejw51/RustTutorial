struct Processor {
   callback: Box<FnMut()>,
}

impl Processor {
   fn set_callback<CB: 'static + FnMut()>(&mut self, c: CB) {
      self.callback = Box::new(c);
   }

   fn process_events(&mut self) {
      (self.callback)();
   }
}

fn simple_callback() {
   println!("hello");
}

fn main() {
   let mut p = Processor {
      callback: Box::new(simple_callback),
   };
   p.process_events();
   let s = "world!".to_string();
   let callback2 = move || println!("hello {}", s);
   p.set_callback(callback2);
   p.process_events();
}
