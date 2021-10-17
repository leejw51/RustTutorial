use serde::{Deserialize, Serialize};
use serde_json;
mod test;
#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "from::string")]
    biguint: u64,

    #[serde(with = "from::string")]
    bigint: i64,
}

// Serde's 'with' attribute allows you to define a serializer and a deserializer in a module.
// You can implement serialize and deserialize as functions in your code, without using modules,
// and use the `deserialize_with` and `serialize_with` attributes instead.
pub mod from {
    // This is just syntactic sugar, so the attribute looks cool like "from::string".
    // You don't need to embed multiple modules.
    pub mod string {
        use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

        // This deserializer was originally written with u64 in mind. Then it was made generic by
        // changing u64 to T everywhere and adding boundaries. Same with the serializer.
        pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Display,
        {
            String::deserialize(deserializer)?
                .parse::<T>()
                .map_err(|e| D::Error::custom(format!("{}", e)))
        }

        pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
            T: std::fmt::Display,
        {
            format!("{}", value).serialize(serializer)
        }
    }
}

fn main() {
    // Deserialize with custom deserializer

    let incomingcustom = r#"
    {
      "biguint": "9007199254740992",
      "bigint": "9007199254740994"
    }
    "#;

    let customresult: MyStruct = serde_json::from_str(&incomingcustom).unwrap();
    println!("biguint deserialized: {}", customresult.biguint);
    println!("bigint deserialized: {}", customresult.bigint);

    // Serialize with custom serializer

    let data = MyStruct {
        biguint: 9007199254740993,
        bigint: 9007199254740995,
    };
    println!("Serialized: {}", serde_json::to_string(&data).unwrap());
    // Prints: Serialized: {"biguint":"9007199254740993","bigint":"9007199254740995"}
}
