struct Processor {
   callback: Box<FnMut(String)->String>,
}

impl Processor {
   fn set_callback<CB: 'static + FnMut(String)->String>(&mut self, c: CB) {
      self.callback = Box::new(c);
   }

   fn process_events(&mut self) {
      let result=(self.callback)("pear".to_string());
      println!("result={}", result);
   }
}

fn simple_callback() {
   println!("hello");
}

fn addString(a: String) -> String
{
  let ret = a+ "_OK";
  ret
}

fn main() {
   let mut p = Processor {
      callback: Box::new(addString),
   };
   p.process_events();
   
   /*
   let s = "world!".to_string();
   let callback2 = move || println!("hello {}", s);
   p.set_callback(callback2);
   p.process_events();*/
}
