mod program;

use program::Program;

fn main() {
    println!("Hello, world!");
    let mut p = Program::default();
    p.process();
}
