extern crate hex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Result;
#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn test1() {
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}

fn test2() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}
fn main() {
    test1();
    test2();
}
