fn check(number: i32, callback: fn(i32) -> bool) {
    for i in 0..number {
        if callback(i) {
            println!("{}", i);
        }
    }
}
fn main() {
    check(10, |a| a % 2 == 0);
}
