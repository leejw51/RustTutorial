use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;
use std::time::Duration;

type Words = Vec<String>;

fn main() {
    let data = Arc::new(Mutex::new(Words::new()));
    {
        let mut data2 = data.lock().unwrap();
        data2.push("apple".to_string());
    }

    let mut threads: Vec<thread::JoinHandle<_>> = vec![];
    for i in 0..3 {
        let data = data.clone();
        let one = thread::spawn(move || {
            for j in 0..10 {
                {
                    let mut data = data.lock().unwrap();
                    data.push(format!("apple {} {}", i, j));
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
        threads.push(one);
    }

    for w in threads {
        w.join().unwrap();
    }

    {
        let mut data2 = data.lock().unwrap();
        for x in &*data2 {
            println!("x={}", x);
        }
    }
    println!("done");
}
