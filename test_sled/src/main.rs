use std::str;
use blake2::{Blake2b, Blake2s, Digest};
use std::time::Instant;
mod test;

fn write_db(count:i32) -> Result<(), failure::Error> {
    let tree = sled::open("./disk")?;
    let start= Instant::now();
    for i in 0..count {
        let mut hasher = Blake2s::new();    
        hasher.input(format!("{}",i));
        let res= hasher.result();

        let mut hasher_value = Blake2s::new();    
        hasher_value.input(format!("data {}",i));
        let res_value= hasher_value.result();
        println!("{} {}", hex::encode(res), hex::encode(res_value));
        tree.insert(res.as_slice(), res_value.as_slice())?;
    }
    
    tree.flush()?;
    println!("time={:?}", start.elapsed());
    Ok(())
}


fn main() {
    write_db(1000).unwrap();
}
