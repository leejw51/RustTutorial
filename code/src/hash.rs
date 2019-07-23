fn main() {
    println!("OK");
    use blake2::{Blake2b, Blake2s, Digest};

    // create a Blake2b object
    let mut hasher = Blake2b::new();
    // write input message
    hasher.input(b"apple");
    hasher.input(b"apple");
    let r = hasher.result();
    println!("{}", hex::encode(r));

    let mut hasher2 = Blake2b::new();
    // write input message
    hasher2.input(b"appleapple");
    let r2 = hasher2.result();
    println!("{}", hex::encode(r2));

}
