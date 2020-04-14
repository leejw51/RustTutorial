use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

#[derive(Default)]
pub struct Worker {
    //  works: Vec<JoinHandle<()>>,
    works: HashMap<String, String>,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            works: HashMap::new(),
        }
    }
    pub fn add(&mut self, newthread: &str) {
        self.works
            .insert(newthread.to_string(), newthread.to_string());
        println!("add current={}", self.works.len());
    }
    pub fn remove(&mut self, removethread: &str) {
        self.works.remove(removethread);
        println!("remove current={}", self.works.len());
    }
    pub fn exist(&self, thread: &str) -> bool {
        self.works.contains_key(thread)
    }
}

pub type WorkerShared = Arc<Mutex<Worker>>;
