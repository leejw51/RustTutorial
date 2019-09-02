use chrono::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;
type WORDS = Vec<String>;

#[derive(Clone,Debug)]
struct ProgramState {
    words: WORDS,
}
impl ProgramState {
    pub fn new()-> Self {
        ProgramState {
            words: vec![],
        }
    }
}


#[derive(Clone, Debug)]
struct Program {
    name: String,   
    words: Vec<String>,
    data: Arc<Mutex<ProgramState>>,
}

impl Program {
    pub fn run_consumer(&self, worker: String) {
        loop {
            sleep(Duration::from_millis(1000));
            {
                let mut data2 = self.data.lock().unwrap();
                if data2.words.len() > 0 {
                    let item = data2.words.pop().unwrap();
                    println!("{} consumed {} ===============================",worker,  item);
                }
                
            }
            
        }
    }
    pub fn run(&self, worker: String) {
        loop {
            sleep(Duration::from_millis(1000));
            // self.words.push(format!("{}-{}", worker,Local::now() ));
            {
                let mut data2 = self.data.lock().unwrap();
                data2.words.push(format!("apple-{}", Local::now()));
                println!("length {}", data2.words.len());
            }
            println!("{} running {}", worker, Local::now());
        }
    }
}
type ProgramShared = Arc<Program>;

fn main() {
    let mut a = ProgramShared::new(Program {
        name: "program".into(),
        words: vec![],
        data: Arc::new(Mutex::new(ProgramState::new())),
    });

    let c = a.clone();
    let c_join = spawn(move || {
        c.run("c".into());
    });
    let d = a.clone();
    let d_join = spawn(move || {
        d.run("d".into());
    });
   

    let a_join = spawn(move || {
        a.run_consumer("a".into());
    });

    c_join.join();
    d_join.join();
    a_join.join();
    println!("OK");
}
