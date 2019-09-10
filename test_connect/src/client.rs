use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

pub fn client() {
    quest::ask("client==========================================\n");
    let addr = "127.0.0.1:8080".parse().unwrap();
    let client = TcpStream::connect(&addr)
        .map(|stream| {
            println!("successfully connected to {}", stream.local_addr().unwrap());
        })
        .map_err(|e| println!("error={}", e));
    tokio::run(client);
}

pub fn client2() {
    quest::ask("client==========================================\n");
    let addr = "127.0.0.1:8080".parse().unwrap();
    let client = TcpStream::connect(&addr)
        .and_then(|stream| {
            println!("created stream");

            io::write_all(stream, "hello world\n").then(|result| {
                println!("wrote to stream; success={:?}", result.is_ok());
                Ok(())
            })
        })
        .map_err(|err| {
            // All tasks must have an `Error` type of `()`. This forces error
            // handling and helps avoid silencing failures.
            //
            // In our example, we are only going to log the error to STDOUT.
            println!("connection error = {:?}", err);
        });
    tokio::run(client);
}
