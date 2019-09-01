use chrono::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;
type WORDS = Vec<String>;

#[derive(Clone, Debug)]
struct Program {
    name: String,
    words: Vec<String>,
    data: Arc<Mutex<WORDS>>,
}

impl Program {
    pub fn run(&self, worker: String) {
        loop {
            sleep(Duration::from_millis(1000));
            // self.words.push(format!("{}-{}", worker,Local::now() ));
            {
                let mut data2 = self.data.lock().unwrap();
                data2.push("apple".to_string());
                println!("length {}", data2.len());
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
        data: Arc::new(Mutex::new(WORDS::new())),
    });

    let c = a.clone();
    let c_join = spawn(move || {
        c.run("c".into());
    });
    let d = a.clone();
    let d_join = spawn(move || {
        d.run("d".into());
    });
    c_join.join();
    d_join.join();
    println!("OK");
}
