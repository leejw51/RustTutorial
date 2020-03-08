mod program;
mod consumer;
mod producer;
use program::Program;


fn main() {
    println!("Hello, world!");
    let mut p = Program::new();
    p.process();
}
