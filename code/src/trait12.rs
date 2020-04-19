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

#[derive(Default, Debug)]
struct Ramen {}
impl Food for Ramen {
    fn eat(&self) {
        println!("eat ramen");
    }
}

struct Program {
    nodes: Vec<Box<Food>>,
}
//Shared references in Rust disallow mutation by default, and Rc is no exception
impl Program {
    pub fn show(&mut self) {
        for node in &self.nodes {
            node.eat();
        }
    }
}
fn main() {
    let mut a: Program = Program { nodes: vec![] };
    // a.node = (Box::new(Apple::default()));
    // a.show();

    // a.node = (Box::new(Ramen::default()));
    a.show();

    println!("ok");
}
