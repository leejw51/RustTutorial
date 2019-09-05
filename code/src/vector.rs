fn main() {
    let mut a = vec![1, 2, 3, 5, 8];
    let mut i = 0;
    for i in 0..a.len() {
        if a[i] % 2 == 0 {
            a[i] = a[i] + 1;
        }
    }
    for x in &a {
        println!("{}", x);
    }
}
