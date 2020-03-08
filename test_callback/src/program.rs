use read_input::prelude::*;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type MyCore = Arc<Mutex<Core>>;

#[derive(Clone)]
pub struct Core {
    quit: bool,
}

impl Core {
    pub fn new() -> Self {
        Core { quit: false }
    }
}

pub struct CoreProducer {
    quit: bool,
    core: Option<MyCore>,
    sender: Sender<String>,
}
impl CoreProducer {
    pub fn new(sender: Sender<String>) -> Self {
        CoreProducer {
            quit: false,
            core: None,
            sender,
        }
    }
    pub fn process(&mut self) {
        loop {
            println!("CoreProducer process");

            let msg=format!("apple ][ {}", chrono::Local::now());
            self.sender.send(msg.to_string());
            std::thread::sleep(std::time::Duration::from_secs(1));

            let b = self.core.as_ref().unwrap().lock().unwrap();

            if b.quit {
                break;
            }
        }
    }
}
pub struct CoreConsumer {
    quit: bool,
    core: Option<MyCore>,
    receiver: Receiver<String>,
}
impl CoreConsumer {
    pub fn new(receiver: Receiver<String>) -> Self {
        CoreConsumer {
            quit: false,
            core: None,
            receiver,
        }
    }

    pub fn process(&mut self) {
        loop {
            println!("CoreConsumer process");
            //std::thread::sleep(std::time::Duration::from_secs(1));
            if let Ok(v)=self.receiver.recv_timeout(std::time::Duration::from_secs(1)) {
                println!("received {}", v);
            }

            let b = self.core.as_ref().unwrap().lock().unwrap();
            if b.quit {
                break;
            }
        }
    }
}

pub struct Program {
    //pub producer: Producer,
    //pub consumer: Consumer,
    pub core: MyCore,
    pub producer: Option<CoreProducer>,
    pub consumer: Option<CoreConsumer>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            core: Arc::new(Mutex::new(Core::new())),
            producer: None,
            consumer: None,
        }
    }

    pub fn process(&mut self) {
        println!("program process");

        let (sender, receiver): (Sender<String>, Receiver<String>) = channel();

        let core1 = Some(self.core.clone());
        let child = thread::spawn(|| {
            let mut p = CoreProducer::new(sender);
            p.core = core1;
            p.process();
        });
        let core2 = Some(self.core.clone());
        let child2 = thread::spawn(|| {
            let mut p = CoreConsumer::new(receiver);
            p.core = core2;
            p.process();
        });

        let a: String = input().msg("input command: ").get();
        println!("OK {}", a);
        {
            self.core.lock().unwrap().quit = true;
        }

        child.join();
        child2.join();
    }

    // add code here
}
