use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
    t: SystemTime,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

fn test1() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}

fn main() {
    let world = World(vec![
        Entity {
            x: 0.0,
            y: 4.0,
            t: SystemTime::now(),
        },
        Entity {
            x: 10.0,
            y: 20.5,
            t: SystemTime::now(),
        },
    ]);

    let encoded: Vec<u8> = bincode::serialize(&world).unwrap();
    println!("encoded = {}", hex::encode(&encoded));
    println!("encoded = {}", encoded.len());

    let decoded: World = bincode::deserialize(&encoded[..]).unwrap();

    assert_eq!(world, decoded);

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&world).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);
    println!("serialized = {}", serialized.len());
}
