use std::rc::Rc;

trait Food {
    fn eat(&self);
}

#[derive(Default, Debug)]
struct Apple {
    label: i32,
}
impl Food for Apple {
    fn eat(&self) {
        println!("eat apple {}", self.label);
    }
}

#[derive(Default,Debug)]
struct Ramen {}
impl Food for Ramen {
    fn eat(&self) {
        println!("eat ramen");
    }
}

#[derive(Default)]
struct Program {
    node: Option<Box<dyn Food>>,
}
//Shared references in Rust disallow mutation by default, and Rc is no exception
impl Program {
   
   pub fn show(&mut self) {
       if self.node.is_some() {
           self.node.as_ref().unwrap().eat();
       }

   }
}
fn main() {
    let mut a: Program = Program::default();
    a.node= Some(Box::new(Apple::default()));
    a.show();

 a.node= Some(Box::new(Ramen::default()));
    a.show();

    println!("ok");
}
