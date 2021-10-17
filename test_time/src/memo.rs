use uint::construct_uint;
use chrono::prelude::*;

pub fn test2()-> Result<(), failure::Error> {
    let now = chrono::Utc::now();
    let a= now. to_rfc3339();
    let b = DateTime::parse_from_rfc3339(&a)?.to_rfc3339();
    let c = DateTime::parse_from_rfc3339(&a)?.to_rfc2822();
    println!("now ={}", now);
    println!("a={}", a);
    println!("b={}", b);
    println!("b={}", c);
    Ok(())
}
use crate::U256;
pub fn test3() {
    let now = chrono::Utc::now();
    let mut a :U256 = U256::from(now.timestamp_nanos());
    let mut b: [u8;32]=[0;32];
    let mut c : U256 = a*a;
    let mut c2: [u8;32]=[0;32];
    a.to_big_endian(&mut b);    
    c.to_big_endian(&mut c2);    
    
    println!("{}",hex::encode(b));
    println!("{:?}", c);
    println!("{}", hex::encode(c2));
}

fn mytest() {

}