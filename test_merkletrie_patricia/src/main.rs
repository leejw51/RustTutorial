mod database;
mod patricia_merkletrie;

use crate::database::Database;
use crate::patricia_merkletrie::Merkletrie;
use crate::patricia_merkletrie::Value;

fn main() {
    env_logger::init();
    let db = Database::new();
    let mut trie = Merkletrie::new(db.clone());
    //trie.load_hex("31f9ae294f11e79fbc0115fac36ca29a0e0697b89de252643bed266103dffe0e8370258e7fd77a6da451a190eff535915d4ff8b024c671c725987d9f5689966b");

    trie.show_roothash();
    trie.put(&hex::decode("abcdef").unwrap(), &Value::new("apple"));
    let m = trie.get(&hex::decode("abcdef").unwrap()).unwrap();
    println!("{}", m.label);
    println!("................");
    trie.show_roothash();
    trie.put(&hex::decode("abcdef").unwrap(), &Value::new("iphone"));
    let m2 = trie.get(&hex::decode("abcdef").unwrap()).unwrap();
    println!("{}", m2.label);

    {
        let roothash = trie.get_roothash();
        let mut trie2 = Merkletrie::new(db.clone());
        trie2.load(&roothash);
        trie2.show_roothash();
        let m2 = trie2.get(&hex::decode("abcdef").unwrap()).unwrap();
        println!("{}", m2.label);
        trie2.show_roothash();
    }
}
