
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);


fn test() {
    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);
    let encoded: Vec<u8> = bincode::serialize(&world).unwrap();
    // 8 bytes for the length of the vector, 4 bytes per float.
    assert_eq!(encoded.len(), 8 + 4 * 4);
    let decoded: World = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(world, decoded);


}

#[derive(Serialize, Deserialize)]
enum Fruit {
    Apple(String),
    Pear(i64),
}


fn test2()->Result<(), failure::Error>
{
    let m= Fruit::Apple("mac".to_string());
    let encoded= bincode::serialize(&m)?;
    println!("encoded={}", hex::encode(&encoded));

    let decoded: Fruit = bincode::deserialize(&encoded)?;
    println!("decoded={}", serde_json::to_string_pretty(&decoded)?);
    Ok(())
}
fn main() {
    test2().unwrap();
}