#![deny(warnings)]

extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::env;
use std::net::SocketAddr;

pub fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>()?;

    let socket = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let (reader, writer) = socket.split();
            let amt = io::copy(reader, writer);
            let msg = amt.then(move |result| {
                match result {
                    Ok((amt, _, _)) => println!("wrote {} bytes", amt),
                    Err(e) => println!("error: {}", e),
                }

                Ok(())
            });

            tokio::spawn(msg)
        });

    tokio::run(done);
    Ok(())
}
