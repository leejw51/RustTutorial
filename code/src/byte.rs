fn change(b: &mut [u8; 10]) {
    for i in 0..b.len() {
        b[i] = 2;
    }
}
fn main() {
    let a = [0; 100];
    let mut b: [u8; 10] = [1, 2, 3, 4, 5, 6, 10, 20, 30, 15];
    change(&mut b);
    for x in &b {
        println!("{}", x);
    }
}
