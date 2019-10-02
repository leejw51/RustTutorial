struct Program {}
impl Program {
    pub fn new() -> Self {
        Program {}
    }
    pub fn run(&self) {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("running {}", chrono::Local::now());
        }
    }
}
fn main() {
    let p = Program::new();
    let handle = std::thread::spawn(move || {
        p.run();
    });
    handle.join();
}
