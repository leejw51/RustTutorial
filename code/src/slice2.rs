
fn main() {
    let mut a : Vec<i32>= Vec::new();
    for i in 0..10 {
        a.push(i*20);
    }

    let b =a[1..3].to_vec();

    for v in b{
        println!("{}", v);
    }
    println!("OK");
}