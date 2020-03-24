use std::str;
use blake2::{Blake2b, Blake2s, Digest};
use std::time::Instant;
use sled::{Batch};
mod test;

fn write_db(count:i32) -> Result<(), failure::Error> {
    let db= sled::open("./disk5")?;
    let tree= db.open_tree("space")?;
    let start= Instant::now();
    for i in 0..count {
        let mut hasher = Blake2s::new();    
        hasher.input(format!("{}",i));
        let res= hasher.result();

        let mut hasher_value = Blake2s::new();    
        hasher_value.input(format!("data {}",i));
        let res_value= hasher_value.result();
       // println!("{} {}", hex::encode(res), hex::encode(res_value));
        tree.insert(res.as_slice(), res_value.as_slice());
    }
    
    tree.flush()?;
    println!("time={:?}", start.elapsed());
    Ok(())
}



fn write_db_batch(count:i32) -> Result<(), failure::Error> {
    let db= sled::open("./disk_batch5")?;
    let tree= db.open_tree("space")?;
    let start= Instant::now();

    let mut batch = Batch::default();
    for i in 0..count {
        let mut hasher = Blake2s::new();    
        hasher.input(format!("{}",i));
        let res= hasher.result();

        let mut hasher_value = Blake2s::new();    
        hasher_value.input(format!("data {}",i));
        let res_value= hasher_value.result();
     //   println!("{} {}", hex::encode(res), hex::encode(res_value));
        batch.insert(res.as_slice(), res_value.as_slice());
    }
    tree.apply_batch(batch)?;
    
    tree.flush()?;
    println!("time={:?}", start.elapsed());
    Ok(())
}

fn main() {
    let count=100000;
    write_db(count).unwrap();
    write_db_batch(count).unwrap();
}
