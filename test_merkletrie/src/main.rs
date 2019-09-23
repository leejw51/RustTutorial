use crate::database::Database;
use crate::merkletrie::Merkletrie;
use bytes::Bytes;
mod database;
mod merkletrie;
fn main() {
    env_logger::init();
    let db = Database::new();
    let mut trie = Merkletrie::new(db);
    //trie.load_hex("31f9ae294f11e79fbc0115fac36ca29a0e0697b89de252643bed266103dffe0e8370258e7fd77a6da451a190eff535915d4ff8b024c671c725987d9f5689966b");

    trie.show_roothash();
    trie.put(
        &Bytes::from(hex::decode("abcdef").unwrap()),
        &Bytes::from(hex::decode("11223344").unwrap()),
    );
    let m = trie
        .get(&Bytes::from(hex::decode("abcdef").unwrap()))
        .unwrap();
    println!("{}", hex::encode(&m));
    println!("................");
    trie.show_roothash();
    trie.put(
        &Bytes::from(hex::decode("abcdef").unwrap()),
        &Bytes::from(hex::decode("1020").unwrap()),
    );
    let m2 = trie
        .get(&Bytes::from(hex::decode("abcdef").unwrap()))
        .unwrap();
    println!("{}", hex::encode(&m2))

    /* let roothash = trie.get_roothash();
    let mut trie2 = Merkletrie::new(db.clone());
    trie2.load(&roothash);
    trie2.show_roothash();
      let m2= trie2.get( &Bytes::from(hex::decode("abcdef").unwrap())).unwrap();
         println!("{}", hex::encode(&m2));*/
}
