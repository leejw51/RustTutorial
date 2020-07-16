use std::sync::{Arc, Mutex};

trait Storage {
    fn write(&self);
}

#[derive(Clone)]
struct DB {}

impl Storage for DB {
    fn write(&self) {
        println!("write");
    }
}

trait MyStorage {
    fn write(&self);
}

struct MyDB<T: Storage> {
    storage: T,
}

impl<T> MyStorage for MyDB<T>
where
    T: Storage,
{
    fn write(&self) {
        println!("write2");
        self.storage.write();
    }
}

#[derive(Clone)]
struct Note {
    storage: Option<Arc<Mutex<dyn MyStorage>>>,
}
fn main() {
    println!("storage");
    let db = DB {};
    let mydb = MyDB { storage: db };

    let mut note = Note {
        storage: Some(Arc::new(Mutex::new(mydb))),
    };
    note.storage.map(|s| s.lock().unwrap().write());
}
