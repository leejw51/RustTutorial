extern crate multiqueue;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time;
extern crate chrono;
use chrono::prelude::*;
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    let p1 = thread::spawn(move || loop {
        let w = time::Duration::from_millis(1000);
        let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
        let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

        thread::sleep(w);
        tx.send(format!("APPLE {:?}", local)).unwrap();
    });
    let p2 = thread::spawn(move || loop {
        let w = time::Duration::from_millis(1000);
        let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
        let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

        thread::sleep(w);
        tx2.send(format!("APPLE2 {:?}", local)).unwrap();
    });

    let worker = thread::spawn(move || loop {
        let b = rx.recv().map(|n| {
            println!("Received {}", n);
        });
    });
    worker.join().unwrap();
    p1.join().unwrap();
    p2.join().unwrap();
}
