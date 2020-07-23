use std::time::Instant;
fn test ()
{
    let a= Instant::now();
    for i in 0..10 {
        println!("{}", i);
    }
    std::thread::sleep( std::time::Duration::from_secs(1));
    println!("time {} milli seocnds",a.elapsed().as_millis());
}
fn main() {
    test();
    println!("Hello, world!");
}
