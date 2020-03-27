use blake2::{Blake2b, Blake2s, Digest};
use sled::Batch;
use std::str;
use std::time::Instant;

pub fn write_db(count: i32) -> Result<(), failure::Error> {
    let db = sled::open("./data/disk5")?;
    let tree = db.open_tree("space")?;
    let start = Instant::now();
    for i in 0..count {
        let mut hasher = Blake2s::new();
        hasher.input(format!("{}", i));
        let res = hasher.result();

        let mut hasher_value = Blake2s::new();
        hasher_value.input(format!("data {}", i));
        let res_value = hasher_value.result();
        // println!("{} {}", hex::encode(res), hex::encode(res_value));
        tree.insert(res.as_slice(), res_value.as_slice());
    }

    tree.flush()?;
    println!("time={:?}", start.elapsed());
    Ok(())
}

pub fn write_db_batch(count: i32) -> Result<(), failure::Error> {
    let db = sled::open("./data/disk_batch5")?;
    let tree = db.open_tree("space")?;
    let start = Instant::now();

    let mut batch = Batch::default();
    for i in 0..count {
        let mut hasher = Blake2s::new();
        hasher.input(format!("{}", i));
        let res = hasher.result();

        let mut hasher_value = Blake2s::new();
        hasher_value.input(format!("data {}", i));
        let res_value = hasher_value.result();
        //   println!("{} {}", hex::encode(res), hex::encode(res_value));
        batch.insert(res.as_slice(), res_value.as_slice());
    }
    tree.apply_batch(batch)?;

    tree.flush()?;
    println!("time={:?}", start.elapsed());
    Ok(())
}

pub fn test2(count: i32) -> Result<(), failure::Error> {
    let db = sled::open("./data/disk5")?;
    let tree = db.open_tree("space")?;
    let start = Instant::now();
    for i in 0..count {
        let mut hasher = Blake2s::new();
        let key: u64 = i as u64;
        let mut key2: [u8; 8] = [0; 8];
        key2.copy_from_slice(&u64::to_be_bytes(key));
        let mut hasher_value = Blake2s::new();
        hasher_value.input(format!("data {}", i));
        let res_value = hasher_value.result();
        // println!("{} {}", hex::encode(res), hex::encode(res_value));
        tree.insert(&key2, res_value.as_slice());
    }

    for k in tree.iter() {
        let key = &k?.0[..];
        let mut key2: [u8; 8] = [0; 8];
        key2.copy_from_slice(key);
        let m = u64::from_be_bytes(key2);
        println!("{}", m);
    }

    tree.flush()?;
    println!("time={:?}", start.elapsed());
    Ok(())
}

pub fn test3(count: i32) -> Result<(), failure::Error> {
    let db = sled::open("./data/disk5")?;
    let tree = db.open_tree("space")?;
    let start = Instant::now();
    for i in 0..count {
        let mut hasher = Blake2s::new();
        //let key = i as u64;
        let key2 = hex::encode(&u64::to_be_bytes(i as u64));
        let mut hasher_value = Blake2s::new();
        hasher_value.input(format!("data {}", i));
        let res_value = hasher_value.result();
        // println!("{} {}", hex::encode(res), hex::encode(res_value));
        tree.insert(&key2, &format!("data={}", i)[..]);
    }

    for k in tree.iter() {
        let (key, value) = k?;
        println!(
            "{}={}",
            std::str::from_utf8(&key)?,
            std::str::from_utf8(&value)?
        );
    }

    tree.flush()?;
    println!("time={:?}", start.elapsed());
    Ok(())
}

use tempfile::tempdir;
    use std::fs::File;
    use std::io::{ Write};
    

pub fn test(count: u32) -> Result<(), failure::Error> {
    
    // Create a directory inside of `std::env::temp_dir()`.
    let dir = tempdir()?;
    
    let file_path = dir.path().join("my-temporary-note.txt");
    let mut file = File::create(&file_path)?;
    writeln!(file, "Brian was here. Briefly.")?;
    drop(file);
    println!("{:?}", file_path);
    let m=std::fs::read_to_string(file_path)?;
    println!("{}", m);
    
    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    
    dir.close()?;
    Ok(())
}
