fn main() {
    let a: Vec<i32> = vec![0i32, 1, 2];

    let s: Vec<i32> = a.into_iter().filter(|x| x.is_positive()).collect();
    println!("{:?}", s);
}
