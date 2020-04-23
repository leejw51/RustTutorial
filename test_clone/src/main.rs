mod test1;
mod test2;
mod test3;

use test3::{Builder, Disk, Websocket};
use test3::{BuilderInterface, DiskInterface, Program2, WebsocketInterface};

struct MyProgram<D, W, B>
where
    D: DiskInterface,
    W: WebsocketInterface,
    B: BuilderInterface,
{
    program: Program2<D, W, B>,
}

impl<D, W, B> MyProgram<D, W, B>
where
    D: DiskInterface,
    W: WebsocketInterface,
    B: BuilderInterface,
{
    pub fn process(&mut self) {
        println!("process");
    }
}
fn main() {
 //   let disk = Box::new(Disk {});
  //  let socket = Box::new(Websocket {});
   // let builder = Box::new(Builder {});
    let program :Program2<Disk,Websocket, Builder>= Program2::default();
    program.client2.disk.boot();
    println!("{:?}", program);
}
