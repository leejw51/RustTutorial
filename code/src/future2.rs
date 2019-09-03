use futures::{Async, Future, Poll};
use std::fmt;
use tokio;
struct HelloWorld;

impl Future for HelloWorld {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        println!("future poll");
        Ok(Async::Ready(()))
    }
}

fn main() {
    println!("future");

    let mut future = HelloWorld {};
    // tokio::run(future);
    future.poll();
}
