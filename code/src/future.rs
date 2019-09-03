use futures::{Async, Future, Poll};
use std::fmt;
use tokio;
struct HelloWorld;

impl Future for HelloWorld {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready("hello world".to_string()))
    }
}

struct Display<T>(T);

impl<T> Future for Display<T>
where
    T: Future,
    T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), T::Error> {
        let value = match self.0.poll() {
            Ok(Async::Ready(value)) => value,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(err) => return Err(err),
        };

        println!("{}", value);
        Ok(Async::Ready(()))
    }
}
fn main() {
    println!("future");

    // let future = Display(HelloWorld);
    // tokio::run(future);
    let mut future2 = HelloWorld {};
    match future2.poll() {
        Ok(Async::Ready(a)) => println!("{}", a),
        Ok(Async::NotReady) => println!("not ready"),
        Err(b) => {
            println!("err");
        }
    }
}
