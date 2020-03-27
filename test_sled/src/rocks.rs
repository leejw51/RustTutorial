use blake2::{Blake2b, Blake2s, Digest};
use rocksdb::{Options, WriteBatch, DB};
use std::str;
use std::time::Instant;

pub fn write_db(count: i32) -> Result<(), failure::Error> {
    let path = "./data/rocks_disk5";
    let db = DB::open_default(path)?;
    let start = Instant::now();
    for i in 0..count {
        let mut hasher = Blake2s::new();
        hasher.input(format!("{}", i));
        let res = hasher.result();

        let mut hasher_value = Blake2s::new();
        hasher_value.input(format!("data {}", i));
        let res_value = hasher_value.result();
        // println!("{} {}", hex::encode(res), hex::encode(res_value));
        db.put(res.as_slice(), res_value.as_slice());
    }

    DB::destroy(&Options::default(), path);
    println!("time={:?}", start.elapsed());
    Ok(())
}

pub fn write_db_batch(count: i32) -> Result<(), failure::Error> {
    let path = "./data/rocks_disk_batch5";
    let db = DB::open_default(path)?;
    let start = Instant::now();
    let mut batch = WriteBatch::default();
    for i in 0..count {
        let mut hasher = Blake2s::new();
        hasher.input(format!("{}", i));
        let res = hasher.result();

        let mut hasher_value = Blake2s::new();
        hasher_value.input(format!("data {}", i));
        let res_value = hasher_value.result();
        // println!("{} {}", hex::encode(res), hex::encode(res_value));
        batch.put(res.as_slice(), res_value.as_slice());
    }
    db.write(batch);
    DB::destroy(&Options::default(), path);
    println!("time={:?}", start.elapsed());
    Ok(())
}

pub fn test(count: i32) -> Result<(), failure::Error> {
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
