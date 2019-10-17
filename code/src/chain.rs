fn main() {
    println!("OK");
    let a = & [10, 20, 30, 40, 50];
    let b = &[60, 70, 80, 0];
    let c= a.iter().chain(b);
    c.for_each( |a| {
        println!("{}",a);
    });
}
