use read_input::prelude::*;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

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
    name: String,
    core: Option<MyCore>,
    sender: Sender<String>,
}
impl CoreProducer {
    pub fn new(sender: Sender<String>, name: String) -> Self {
        CoreProducer {
            name,
            core: None,
            sender,
        }
    }
    pub fn process(&mut self) {
        loop {
            println!("CoreProducer process {}", self.name);

            let msg = format!("apple ][ {}", chrono::Local::now());
            self.sender.send(msg).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));

            let b = self.core.as_ref().unwrap().lock().unwrap();

            if b.quit {
                break;
            }
        }
    }
}
pub struct CoreConsumer {
    core: Option<MyCore>,
    receiver: Receiver<String>,
}
impl CoreConsumer {
    pub fn new(receiver: Receiver<String>) -> Self {
        CoreConsumer {
            core: None,
            receiver,
        }
    }

    pub fn process(&mut self) {
        loop {
            println!("CoreConsumer process");
            //std::thread::sleep(std::time::Duration::from_secs(1));
            if let Ok(v) = self
                .receiver
                .recv_timeout(std::time::Duration::from_secs(1))
            {
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

    pub fn add_producer(
        &self,
        nodes: &mut Vec<JoinHandle<()>>,
        core: MyCore,
        sender: Sender<String>,
        name: &str,
    ) {
        let core1 = Some(core);
        let name1 = name.to_string();
        let child = thread::spawn(|| {
            let mut p = CoreProducer::new(sender, name1);
            p.core = core1;
            p.process();
        });
        nodes.push(child);
    }

    pub fn set_consumer(
        &self,
        nodes: &mut Vec<JoinHandle<()>>,
        core: MyCore,
        receiver: Receiver<String>,
    ) {
        let core1 = Some(core);
        let child = thread::spawn(|| {
            let mut p = CoreConsumer::new(receiver);
            p.core = core1;
            p.process();
        });
        nodes.push(child);
    }

    pub fn process(&mut self) {
        println!("program process");

        let (sender, receiver): (Sender<String>, Receiver<String>) = channel();

        let mut threads: Vec<JoinHandle<()>> = vec![];
        self.add_producer(&mut threads, self.core.clone(), sender.clone(), "superman");
        self.add_producer(&mut threads, self.core.clone(), sender.clone(), "batman");
        self.set_consumer(&mut threads, self.core.clone(), receiver);

        let a: String = input().msg("input command: ").get();
        println!("OK {}", a);
        {
            self.core.lock().unwrap().quit = true;
        }

        for x in threads {
            x.join().unwrap();
        }
        println!("done");
    }

    // add code here
}
