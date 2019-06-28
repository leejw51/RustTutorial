fn main() {
    let a: u64 = 20;
    let b = a.to_string();
    let c = b.parse::<u64>().unwrap();
    println!("amount={:?}", a);
    println!("amount={:?}", b);
    println!("amount={:?}", c);
    assert!(a == c);
}