use std::fs::File;
use std::io::prelude::*;
fn process(a: i32) -> Result<String, String> {
    println!("process={}", a);
    let mut f = File::open("test.txt").map_err(|_e| "Error".to_string())?;
    let mut m = String::new();
    f.read_to_string(&mut m).map_err(|_e| "Error".to_string())?;
    Ok(m)
}
fn main() {
    let a = process(100);
    println!("{}", a.unwrap());
}
