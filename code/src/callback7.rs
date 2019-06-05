use std::rc::Rc;
use std::sync::Arc;
use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Weak;

trait NetworkCallback {
   fn onRead(&mut self);
}
trait NetworkServer {
   fn run(&mut self);
   fn read(&mut self, buf: String);
}

struct App {
    net: Arc<RefCell<NetworkCore>>
}
struct NetworkCore {
   version: String,
   app: Weak<RefCell<App>>,
}

//--------------------------------------------------------------
impl NetworkServer for NetworkCore {
   fn run(&mut self) {
      println!("network server run");
   }
   fn read(& mut self, buf: String) {
      println!("read={}", buf);
      self.app.upgrade().unwrap().
      borrow_mut().onRead();
   }
}


impl NetworkCallback for App {
   fn onRead(&mut self) {
      println!("App onRead");
   }
}
fn main() {

  // create===================
  let mut core = Arc::new(RefCell::new(NetworkCore {
    version: "1.0.0".to_string(),
    app: Weak::new(),
  }));
  let mut app = Arc::new(RefCell::new(App {net: core.clone()}));
  let app2 = Arc::downgrade(&app);
  // link============================
  core.borrow_mut().app= app2;
  core.borrow_mut().run();
  core.borrow_mut().read("apple ][".to_string());
}
