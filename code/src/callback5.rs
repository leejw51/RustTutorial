use std::rc::Rc;
use std::sync::Arc;
use std::cell::Cell;
use std::cell::RefCell;

struct App {}
struct NetworkCore {
   version: String,
   app: Arc<RefCell<App>>,
}

trait NetworkCallback {
   fn onRead(&mut self);
}
trait NetworkServer {
   fn run(&mut self);
   fn read(&mut self, buf: String);
}

//--------------------------------------------------------------
impl NetworkServer for NetworkCore {
   fn run(&mut self) {
      println!("network server run");
   }
   fn read(& mut self, buf: String) {
      println!("read={}", buf);
      self.app.borrow_mut().onRead();
   }
}


impl NetworkCallback for App {
   fn onRead(&mut self) {
      println!("App onRead");
   }
}
fn main() {
   let mut app = Arc::new(RefCell::new(App {}));
   let mut core = Arc::new(RefCell::new(NetworkCore {
      version: "1.0.0".to_string(),
      app: app,
   }));
   core.borrow_mut().run();
   core.borrow_mut().read("apple ][".to_string());
}
