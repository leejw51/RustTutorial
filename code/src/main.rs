use std::rc::Rc;
struct App {}
struct NetworkCore {
   version: String,
   app: Box<App>,
}

trait NetworkCallback {
   fn onRead(&mut self);
}
trait NetworkServer {
   fn run(&mut self);
   fn read(&mut self, buf: String);
}
impl NetworkServer for NetworkCore {
   fn run(&mut self) {
      println!("network server run");
   }
   fn read(& mut self, buf: String) {
      println!("read={}", buf);
      self.app.onRead();
   }
}


impl NetworkCallback for App {
   fn onRead(&mut self) {
      println!("App onRead");
   }
}
fn main() {
   let mut app = Box::new(App {});
   let mut core = Box::new(NetworkCore {
      version: "1.0.0".to_string(),
      app: app,
   });
   core.run();
   core.read("apple ][".to_string());
}
