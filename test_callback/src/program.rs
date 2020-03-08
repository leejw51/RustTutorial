use read_input::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

use super::producer::Producer;
use super::consumer::Consumer;
use std::thread;

type MyProducer=Arc<Mutex<dyn Producer>>;
type MyConsumer=Arc<Mutex<dyn Consumer>>;
type MyCore= Arc<Mutex<Core>>;

#[derive(Clone)]
pub struct Core {
    quit: bool,
}

impl Core {
    pub fn new()->Self {
        Core {
            quit: false,
        }
    }
}

pub struct CoreProducer {
    quit: bool,
    core: Option<MyCore>,
}
impl CoreProducer {
    pub fn new()->Self {
        CoreProducer {

            quit: false,
            core:None,
        }
    }
    pub fn process(&mut self) {
        while true {
        println!("CoreProducer process");
    
        
        std::thread::sleep(std::time::Duration::from_secs(1));

        let b= self.core.as_ref().unwrap().lock().unwrap();
        if b.quit  {
            break;
        }

        }
    }
  


}
pub struct CoreConsumer {
    quit: bool,
    core: Option<MyCore>,
    
}
impl CoreConsumer {
    pub fn new()->Self {
        CoreConsumer {

            quit: false,
            core: None,
        }
    }

    pub fn process(&mut self) {
        while true {
        println!("CoreConsumer process");
        std::thread::sleep(std::time::Duration::from_secs(1));

        let b= self.core.as_ref().unwrap().lock().unwrap();
        if b.quit  {
            break;
        }
        }
    }

}

impl Producer for CoreProducer {
    fn push(&self, data: &str)
    {
        println!("push {}", data);
    }

}

pub struct Program {
    //pub producer: Producer,
    //pub consumer: Consumer,
    pub core: MyCore,
    pub producer: Option<  CoreProducer>,
    pub consumer: Option< CoreConsumer>,
}


impl  Program {
    pub fn new()-> Self {
        Program {
            core: Arc::new(Mutex::new(Core::new())),
            producer: None,
            consumer: None,
        }
    }
    pub fn process(& mut  self) {
        println!("program process");
   
    
        let core1= Some(self.core.clone());
        let child=thread::spawn( || {            
            let mut p = CoreProducer::new();              
            p.core= core1;
            p.process();
        });
        let core2= Some(self.core.clone());
        let child2=thread::spawn( || {            
            let mut p = CoreConsumer::new();
            p.core= core2;
            p.process();
        });
       

        let a: String = input().msg("input command: ").get();
        println!("OK {}", a);
        {
            self.core.lock().unwrap().quit=true;
            
        }

        

        child.join();
        child2.join();
    }
    
    // add code here
}