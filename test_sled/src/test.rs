use std::str;

pub fn test_db() -> Result<(), failure::Error> {
    let tree = sled::open("./disk")?;
    // set and get
    tree.insert(b"ok", b"apple".to_vec())?;
    tree.insert(b"ok", b"apple5".to_vec())?;
    println!(
        "{:?}",
        str::from_utf8(&tree.get(b"ok").unwrap_or_default().unwrap_or_default())
            .unwrap_or_default()
    );
    if let Some(found) = tree.get(b"ok")? {
        println!("{}", str::from_utf8(&found)?);
    }
    tree.flush()?;
    Ok(())
}
