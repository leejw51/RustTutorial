use chrono::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;
type BLOCKS = Vec<String>;

#[derive(Clone, Debug)]
struct CoreData {
    words: BLOCKS,
}
impl CoreData {
    pub fn new() -> Self {
        CoreData { words: vec![] }
    }
}

#[derive(Clone, Debug)]
struct Core {
    name: String,
    words: Vec<String>,
    data: Arc<Mutex<CoreData>>,
}

impl Core {
    pub fn new() -> Self {
        Core {
            name: "program".into(),
            words: vec![],
            data: Arc::new(Mutex::new(CoreData::new())),
        }
    }
    pub fn run_consumer(&self, worker: String, receiver: Receiver<String>) {
        loop {
            //sleep(Duration::from_millis(1000));
            let found = receiver.recv_timeout(Duration::from_millis(500));
            if let Ok(found_one) = found {
                println!("consumed  {}", found_one);
            }
            {
                let mut data2 = self.data.lock().unwrap();
                if data2.words.len() > 0 {
                    let item = data2.words.pop().unwrap();
                
                }
            }
        }
    }
    pub fn run(&self, worker: String, sender: Sender<String>) {
        loop {
            sleep(Duration::from_millis(2000));

            {
                let mut data2 = self.data.lock().unwrap();
                data2
                    .words
                    .push(format!("{} apple-{}", worker, Local::now()));
                sender
                    .send(format!("{} strawberry-{}", worker, Local::now()))
                    .unwrap();
            }
        }
    }
}

struct Program {
    core: Core,
}

impl Program {
    pub fn new() -> Self {
        Program { core: Core::new() }
    }

    pub fn run(&mut self) {
        let a =self.core.clone();
        let (sender, receiver) = channel::<String>();
        let sender2 = sender.clone();
        let sender3 = sender.clone();

        let c = a.clone();
        let c_join = spawn(move || {
            c.run("c".into(), sender2);
        });
        let d = a.clone();
        let d_join = spawn(move || {
            d.run("d".into(), sender3);
        });

        let a_join = spawn(move || {
            a.run_consumer("a".into(), receiver);
        });

        c_join.join();
        d_join.join();
        a_join.join();
        println!("OK");
    }
}
fn main() {
    let mut p = Program::new();
    p.run();
}
