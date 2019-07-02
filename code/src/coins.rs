use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct Coin {
    secs: u64,
}

impl Coin {
    pub fn new(s: u64) -> Coin {
        Coin { secs: s }
    }
}

use std::fmt;

use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};

impl<'de> Deserialize<'de> for Coin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Secs,
        };

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "secs" => Ok(Field::Secs),

                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct CoinVisitor;

        impl<'de> Visitor<'de> for CoinVisitor {
            type Value = Coin;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Coin")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Coin, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let secs = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let nanos = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Coin::new(secs))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Coin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut secs = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Secs => {
                            if secs.is_some() {
                                return Err(de::Error::duplicate_field("secs"));
                            }
                            secs = Some(map.next_value()?);
                        }
                    }
                }
                let secs = secs.ok_or_else(|| de::Error::missing_field("secs"))?;
                Ok(Coin::new(secs))
            }
        }

        const FIELDS: &'static [&'static str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Coin", FIELDS, CoinVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

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
    let d = Coin::new(10);
    let serialized = serde_json::to_string(&d).unwrap();

    println!("serialized = {}", serialized);
    // Prints deserialized = Point { x: 1, y: 2
    let deserialized: Coin = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);
}
