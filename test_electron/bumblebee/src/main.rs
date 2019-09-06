use std::thread::sleep;
use std::time::Duration;
fn main() {
    loop {
        sleep(Duration::from_millis(1000));
        println!("...");
    }
}
