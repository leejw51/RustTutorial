#[derive(Default, Clone, Debug)]
struct Apple {}

impl Drop for Apple {
    fn drop(&mut self) {
        println!("apple drop!");
    }
}
fn main() {
    let a = Apple::default();
}
