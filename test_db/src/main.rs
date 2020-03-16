use blake2::{Blake2b, Blake2s, Digest};

fn test_sled()-> Result<(), String> {
    let tree = sled::open("./sled_storage").map_err(|e| e.to_string())?;
    let mut i: u64=0;
    loop 
    {
        let mut hasher = Blake2s::new();
        let key= format!("{}",i);
        hasher.input(key.as_bytes());

        let mut hasher_data= Blake2b::new();
        let data= format!("data {}",i*100);
        hasher_data.input(data.as_bytes());

        let key_res = hasher.result();
        let data_res = hasher_data.result();
        println!("{} {} {}",i,hex::encode(&key_res[..]), hex::encode(&data_res[..]));
        tree.insert(&key_res[..], &data_res[..]);
        i=i+1;
        println!("i={}",i);
    }
    
    Ok(())
}
fn main() {
   
    println!("Hello, world!");
    test_sled();
}
