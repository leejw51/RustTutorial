extern crate multiqueue;
use std::{thread, time};

fn main() {
    println!("test disruptor");
    let (send, recv) = multiqueue::broadcast_queue(4);

    for i in 0..2 {
        // or n
        let cur_recv = recv.add_stream();

        let stream_consumer = cur_recv.clone();
        thread::spawn(move || {
            for val in stream_consumer {
                println!(" core {} consumer got {} ", i, val);
                /*if (i == 0) {
                    if (val % 2 == 0) {
                        println!(" core {} consumer process {} ", i, val);
                    }
                } else {
                    if (val % 2 != 0) {
                        println!(" core {} consumer process {} ", i, val);
                    }
                }*/
            }
        });

        // cur_recv is dropped here
    }

    // Take notice that I drop the reader - this removes it from
    // the queue, meaning that the readers in the new threads
    // won't get starved by the lack of progress from recv
    recv.unsubscribe();

    for i in 0..10 {
        // Don't do this busy loop in real stuff unless you're really sure
        loop {
            if send.try_send(i).is_ok() {
                break;
            }
        }
    }
    let waitTime = time::Duration::from_millis(2000);
    thread::sleep(waitTime);

    drop(send);
}
