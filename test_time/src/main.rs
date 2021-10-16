use std::time::{Duration, SystemTime};
use std::thread::sleep;
use chrono::prelude::*;
use chrono::offset::LocalResult;

fn test() {
    let now = chrono::Utc::now();
   
   let a = now.timestamp_nanos();
   let a1= a/(1_000_000_000);
   let a2:u32= (a%(1_000_000_000 as i64)) as u32;
   let b: DateTime<Utc> = DateTime::<Utc>::from_utc(
       NaiveDateTime::from_timestamp(a1, a2), Utc);
   println!("now {:?}", a);
   println!("now {:?} {:?}", b, now);
   assert!(b==now);

}
fn main() {
    let now = chrono::Utc::now();
    let a= now. to_rfc3339();
    let b = DateTime::parse_from_rfc3339(&a).unwrap().to_rfc3339();
    let c = DateTime::parse_from_rfc3339(&a).unwrap().to_rfc2822();
    println!("now ={}", now);
    println!("a={}", a);
    println!("b={}", b);
    println!("b={}", c);

   
}