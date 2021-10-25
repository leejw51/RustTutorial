use bson::{spec::BinarySubtype, Bson};

use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    #[serde(serialize_with = "self::serialize_bytes_as_binary")]
    #[serde(deserialize_with = "self::deserialize_bytes_as_binary")]
    bytes: Vec<u8>,
    name: String,
}

fn deserialize_bytes_as_binary<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    match Bson::deserialize(deserializer) {
        Ok(Bson::Binary(_binary)) => Ok(_binary.bytes),

        Ok(..) => Err(Error::invalid_value(Unexpected::Enum, &"Bson::Binary")),

        Err(e) => Err(e),
    }
}

fn serialize_bytes_as_binary<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let b = bson::Binary {
        subtype: BinarySubtype::Generic,
        bytes: bytes.to_vec(),
    };
    let binary = Bson::Binary(b);

    binary.serialize(serializer)
}

pub fn test_binary_bson() -> Result<(), failure::Error> {
    let myitem = Item {
        bytes: vec![0xff; 100],
        name: "apple".into(),
    };

    let mybson = bson::to_bson(&myitem)?;
    let mybytes = bson::to_vec(&mybson)?;

    println!("encoded= {:?} ", mybson);
    println!(
        "encoded in {} hex= {} ",
        mybytes.len(),
        hex::encode(&mybytes)
    );
    let foo: Item = bson::from_bson(mybson)?;
    println!("decoded= {:?}", foo);
    Ok(())
}
