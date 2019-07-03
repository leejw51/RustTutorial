#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct TeamSize(pub i64, pub i64);

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    pub size: TeamSize,
}

// private
#[derive(Serialize, Deserialize)]
struct TeamSizeHelper {
    min: i64,
    max: i64,
}

impl Serialize for TeamSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TeamSizeHelper {
            min: self.0,
            max: self.1,
        }
        .serialize(serializer)

    }
}



impl<'de> Deserialize<'de> for TeamSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|TeamSizeHelper { min, max }| TeamSize(min, max))
    }
}

fn main() {
    let j = r#" { "size": { "min": 2, "max": 15 } } "#;

    let m: Match = serde_json::from_str(j).unwrap();
    println!("{:?}", m);

    let j = serde_json::to_string(&m).unwrap();
    println!("{}", j);
}
