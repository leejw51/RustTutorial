
mod memo;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use chrono::prelude::*;
use chrono::offset::LocalResult;

#[macro_use] extern crate failure;
use uint::construct_uint;
construct_uint! {
	pub struct U256(4);
}

#[macro_use] extern crate uint;



fn test() -> Result<(), failure::Error> {
    let now = chrono::Utc::now();
   
   let a = now.timestamp_nanos();
   let a1= a/(1_000_000_000);
   let a2:u32= (a%(1_000_000_000 as i64)) as u32;
   let b: DateTime<Utc> = DateTime::<Utc>::from_utc(
       NaiveDateTime::from_timestamp(a1, a2), Utc);
   println!("now {:?}", a);
   println!("now {:?} {:?}", b, now);
   assert!(b==now);
   Ok(())

}



fn main() {   
    memo::test3();
}