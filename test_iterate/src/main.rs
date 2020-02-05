fn main() {
    let a = vec![10, 20, 300];
    for (x, item) in a.iter().enumerate() {
        if (*item == 20) {
            println!("found {} {}", x, item);
        }
        println!("{} {}", x, item);
    }
}
