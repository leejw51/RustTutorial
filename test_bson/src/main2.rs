use bson::{bson, doc, oid, Bson, DateTime, Document};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
    phones: Vec<String>,
}

fn main2() {
    // Some BSON input data as a `Bson`.
    let bson_data: Bson = bson!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let mut person: Person = bson::from_bson(bson_data).unwrap();

    println!("Redacting {}'s record.", person.name);
    person.name = "REDACTED".to_string();

    let redacted_bson = bson::to_bson(&person).unwrap();
}

fn test() {
    let string = Bson::String("hello world".to_string());
    let int = Bson::Int32(5);
    let array = Bson::Array(vec![Bson::Int32(5), Bson::Boolean(false)]);

    let string: Bson = "hello world".into();
    let int: Bson = 5i32.into();

    let string = bson!("hello world");
    let int = bson!(5);
    let array = bson!([5, false]);
}

fn test2() {
    let mut bytes = bson!(vec![Bson::Int32(5); 24]);
    let apple = doc! {
        "name":"apple][",
        "price":20000,
        "denom":"usd",
        "image": bytes
    };

    let doc = doc! {
       "string": "string",
       "bool": true,
       "i32": 5,
       "doc": { "x": true },
       "apple": apple
    };

    // attempt get values as untyped Bson
    let none = doc.get("asdfadsf"); // None
    let value = doc.get("string"); // Some(&Bson::String("string"))

    // attempt to get values with explicit typing
    let string = doc.get_str("string"); // Ok("string")
    let subdoc = doc.get_document("doc"); // Some(Document({ "x": true }))
    let error = doc.get_i64("i32"); // Err(...)
    let a = doc.to_string();

    let b = bson::to_vec(&doc).unwrap();
    println!("{}", a);
    println!("{}", hex::encode(&b));
}

fn test3() {
    let doc = doc! {
        "bool": true,
        "fruits":vec![Bson::Int32(0x00bbffff);10]
    };

    // attempt get values as untyped Bson
    let a = doc.to_string();
    let b = bson::to_vec(&doc).unwrap();
    println!("{}", a);
    println!("{}", hex::encode(&b));
}

fn test4() {
    crate::binary::test_binary_bson();
}

fn test5() -> Result<(), failure::Error> {
    let mut lines: Vec<Document> = vec![];
    for i in 0..10 {
        let id = (i as i128) * 10000000000000000000000;
        let b = doc! {
            "writer":"steve",
            "msg":"iphone",
            "id":id.to_string(),
        };
        lines.push(b)
    }
    let a = doc! {
        "id":"1",
        "name":"apple][",
        "data":"sampledata",
        "lines": lines
    };
    println!("{}", serde_json::to_string_pretty(&a).unwrap());
    Ok(())
}

fn test6() -> Result<(), failure::Error> {
    let a = doc! {
        "id":"1",
        "name":"apple][",
        "data":"sampledata",

    };
    let myjson = serde_json::to_string_pretty(&a)?;
    let myobj: Map<String, Value> = serde_json::from_str(&myjson)?;
    let mybson = bson::to_document(&myobj)?;
    println!("{}", myjson);
    println!("myobj= {:?}", myobj);
    println!("mybson= {:?}", mybson);
    Ok(())
}
