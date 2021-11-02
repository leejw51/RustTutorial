use bson::{bson, doc, oid, Bson, DateTime, Document};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::convert::TryFrom;
mod binary;
mod main2;

fn test_json_to_bson() -> Result<(), failure::Error> {
    let a = json! ({
        "id":"1",
        "name":"apple][",
        "data":"sampledata",
        "numbers": [
            100,
            200,300,400,500,600,700,800
        ]
    });
    let myjson = serde_json::to_string_pretty(&a)?;
    let myobj: Map<String, Value> = serde_json::from_str(&myjson)?;
    let mybson = bson::to_document(&myobj)?;
    let mybsonbin = bson::to_vec(&mybson)?;
    println!("length {} {}", myjson.as_bytes().len(), myjson);
    println!("myobj= {:?}", myobj);
    println!("mybson= {:?}", mybson);
    println!(
        "mybson lengh {} hex= {}",
        mybsonbin.len(),
        hex::encode(&mybsonbin)
    );
    Ok(())
}

fn main() {
    test_json_to_bson();
}
