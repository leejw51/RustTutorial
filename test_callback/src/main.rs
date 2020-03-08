mod consumer;
mod producer;
mod program;
use program::Program;

fn main() {
    println!("Hello, world!");
    let mut p = Program::new();
    p.process();
}
