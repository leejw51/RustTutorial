use chrono::prelude::{Local, Utc};
use std::{thread, time};
use std::sync::mpsc::{channel, Sender, Receiver};

type SendQueue=Sender<String>;
type RecvQueue=Receiver<String>;
struct Worker {
    is_quit: bool,
    count: i64,
    recv: Option<RecvQueue>,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            is_quit: false,
            count: 0,
            recv: None,
        }
    }
    pub fn process(&mut self ) {
        while !self.is_quit {
            // println!("worker run {}", Local::now());
            
           
              match self.recv.as_mut().unwrap().recv_timeout(time::Duration::from_millis(1000)) {
                  Ok(a)=>{
                      println!("woker {}", a);
                  }
                  Err(b)=>{}
              }
        }
    }

  
}



struct Program {
    w: Worker,
    send: Option<SendQueue>
}

impl Program {
    pub fn run(&mut self) {
        
        
        let (sender, receiver): (SendQueue, RecvQueue) = channel();
        self.send = Some(sender);
        thread::spawn(||  {
            let mut w = Worker::new();
            w.recv = Some(receiver);
            w.process();
        });

        loop {
            thread::sleep(time::Duration::from_millis(1000));
            println!("program run {}", Local::now());
             self.send.as_mut().unwrap().send(format!("hello {}", Local::now()));
        }
    }
}

fn main() {
    let mut p = Program { w: Worker::new(), send: None };
    p.run();
}
