use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
#[derive(Debug)]
struct Fruit {
    price: i128,
    name: String,
}

// box: single ownership
fn test_box() -> Result<()> {
    let mut a = Box::new(Fruit {
        price: 10,
        name: "Apple".to_string(),
    });
    println!("{:?}", a);
    println!("change..");
    a.name = "Banana".to_string();
    println!("{:?}", a);
    Ok(())
}

// Rc: multiple reference for single thread
// modified with RefCell
fn test_rc() -> Result<()> {
    let b = RefCell::new(Fruit {
        price: 10,
        name: "Apple".to_string(),
    });
    let a = Rc::new(b);
    let c = a.clone();
    println!("{:?}", a);
    println!("change..");
    a.borrow_mut().name = "Banana".to_string();

    println!("a {:?}", a);
    println!("c {:?}", c);
    Ok(())
}

// Arc: multiple reference for multi thread
// modified with Lock
fn test_arc() -> Result<()> {
    let b = Mutex::new(Fruit {
        price: 10,
        name: "Apple".to_string(),
    });
    let a = Arc::new(b);
    let c = a.clone();

    println!("{:?}", a);
    println!("change..");
    a.lock().expect("get lock").name = "Banana".to_string();

    println!("a {:?}", a);
    println!("c {:?}", c);
    Ok(())
}
fn main() {
    println!("-----------------------------------------------");
    test_box();
    println!("-----------------------------------------------");
    test_rc();
    println!("-----------------------------------------------");
    test_arc();
}
