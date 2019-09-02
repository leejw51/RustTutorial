use chrono::prelude::*;
use multiqueue::BroadcastReceiver;
use multiqueue::BroadcastSender;
use multiqueue::MPMCReceiver;
use multiqueue::MPMCSender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::thread::spawn;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Clone, Debug)]
struct ProgramData {
    name: String,
    words: Vec<String>,
}
impl ProgramData {
    pub fn new() -> Self {
        ProgramData {
            name: "".into(),
            words: vec![],
        }
    }
}
type ProgramShared = Arc<Mutex<ProgramData>>;

#[derive(Clone, Debug)]
struct Program {
    data: ProgramShared,
}
impl Program {
    pub fn new() -> Self {
        Program {
            data: Arc::new(Mutex::new(ProgramData::new())),
        }
    }
    pub fn run(&self) -> JoinHandle<()> {
        spawn(move || loop {
            //println!("program running {}", Local::now());
            sleep(Duration::from_millis(1000));
        })
    }

    pub fn run_miner(&self, sender: BroadcastSender<String>) -> JoinHandle<()> {
        spawn(move || {
            let mut count: u64 = 1;
            loop {
                let a = format!("block_{}", count);
                println!("miner {}==========================", a);
                sender.try_send(a).unwrap();
                count += 1;
                sleep(Duration::from_millis(1000));
            }
        })
    }

    pub fn run_verifier(&self, receiver: BroadcastReceiver<String>) -> JoinHandle<()> {
        spawn(move || loop {
            if let Ok(a) = receiver.try_recv() {
                println!("verifier {}", a);
            } else {
                sleep(Duration::from_millis(10));
            }
        })
    }

    pub fn run_writer(&self, receiver: BroadcastReceiver<String>) -> JoinHandle<()> {
        spawn(move || loop {
            if let Ok(a) = receiver.try_recv() {
                println!("writer {}", a);
            } else {
                sleep(Duration::from_millis(10));
            }
        })
    }
}

fn main() {
    let program = Program::new();

    let (send, recv): (BroadcastSender<String>, BroadcastReceiver<String>) =
        multiqueue::broadcast_queue(10);

    let miner = program.clone();
    let miner_join = miner.run_miner(send);
    let verifier = program.clone();
    let verifier_join = verifier.run_verifier(recv.add_stream());
    let writer = program.clone();
    let writer_join = writer.run_writer(recv.add_stream());
    let main_join = program.run();

    miner_join.join().unwrap();
    verifier_join.join().unwrap();
    writer_join.join().unwrap();
    main_join.join().unwrap();
}
