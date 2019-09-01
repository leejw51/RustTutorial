use std::thread;
use std::time;
use std::sync::Arc;

struct Worker {

}

impl Worker {

}

struct Program {

}

impl Program {
   
    pub fn process(&'static  self) {
        println!("progrm processing");
    }
    pub fn run(&'static  self){
        let a=thread::spawn( move || {
            self.process();
        });
        a.join();

    
    }
}
fn main(){
    
    let q: &'static Program= &Program{};
    q.run();
    println!("OK");
}