struct Processor<CB>
where
   CB: FnMut(),
{
   callback: CB,
}

impl<CB> Processor<CB>
where
   CB: FnMut(),
{
   fn set_callback(&mut self, c: CB) {
      self.callback = c;
   }

   fn process_events(&mut self) {
      (self.callback)();
   }
}

fn main() {
   let s = "world!".to_string();
   let callback = || println!("hello {}", s);
   let mut p = Processor { callback: callback };
   p.process_events();
}