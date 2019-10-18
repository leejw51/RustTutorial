
fn main() {
    let x: &[u8] = &[1u8, 2, 3];
    println!("{:?}", x);

    let y: &[u8; 3] = &[1u8, 2, 3];
    println!("{:?}", y);
}
