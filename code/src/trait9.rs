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
    nodes: Vec<Box<dyn Food>>,
}
//Shared references in Rust disallow mutation by default, and Rc is no exception
impl Program {
    pub fn process(&mut self) {
        self.nodes.push(Box::new(Apple::default()));
        self.nodes.push(Box::new(Apple::default()));
        self.nodes.push(Box::new(Apple::default()));
        self.nodes.push(Box::new(Ramen::default()));
    }
    pub fn show(&self) {
        for x in &self.nodes {
           x.eat();
        }
    }
}
fn main() {
    let mut a: Program = Program::default();

    a.process();
    a.show();

    println!("ok");
}
