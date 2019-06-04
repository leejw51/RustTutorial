struct Processor<CB>
where
   CB: FnMut(u32) -> u32,
{
   callback: CB,
}

impl<CB> Processor<CB>
where
   CB: FnMut(u32) -> u32,
{
   fn set_callback(&mut self, c: CB) {
      self.callback = c;
   }

   fn process_events(&mut self) {
      let result = (self.callback)(200);
      println!("result={}", result);
   }
}

fn main() {
   let s = "world!".to_string();
   let callback = |a| a + 100;
   let mut p = Processor { callback: callback };
   p.process_events();
}