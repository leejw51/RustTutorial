use std::thread::*;
use std::thread;
use std::time::*;
use std::sync::Arc;
#[derive(Clone,Debug)]
struct Program {
}

impl Program {
    pub fn run(&self) {
        sleep( Duration::from_millis(1000));
        println!("run {}", chrono::prelude::Local::now());
    }
}
type ProgramShared= Arc<Program>;

fn main(){
    let a=ProgramShared::new(Program{});
    let b=a.clone();
    a.run();
    let c= a.clone();
    let c_join=thread::spawn(move || {
        
        c.run();
    });
     let d= a.clone();
    let d_join=thread::spawn(move || {
        d.run();
    });
    c_join.join();
    d_join.join();
    println!("OK");
}