/*
hexa sparse merkletrie
written by Jongwhan Lee
*/

use bytes;
use bytes::Bytes;
use log::debug;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

pub trait MerkletrieDatabase {
    fn compute_hash(&self, data: &[u8]) -> Bytes;
    fn write(&self, key: &[u8], data: &[u8]);
    fn read(&self, key: &[u8]) -> Option<Vec<u8>>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Node {
    pub address: Bytes,
    pub children: BTreeMap<Bytes, Bytes>, // key: address, value: Node Hash
    pub value: Bytes,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Merkletrie<T>
where
    T: MerkletrieDatabase,
{
    database: T,
    root: Node,
}

impl<T> Merkletrie<T>
where
    T: MerkletrieDatabase,
{
    pub fn new(database: T) -> Self {
        debug!("merkletrie created");
        Merkletrie {
            database,
            root: Node::default(),
        }
    }

    pub fn load(&mut self, hash: &Bytes) {
        let node_found = self.read_node(&hash).unwrap();
        self.root = node_found;
    }

    pub fn load_hex(&mut self, hash: &str) {
        self.load(&Bytes::from(hex::decode(hash).unwrap()));
    }

    pub fn initialize(&self) {
        debug!("merkletrie initialized");
    }

    // encoded, hash
    fn get_hash(&self, n: &Node) -> Bytes {
        self.get_encoded_hash(n).1
    }

    // encoded, hash
    #[allow(dead_code)]
    fn show_node(&self, n: &Node) {
        debug!(
            "hash={} json={}",
            hex::encode(self.get_hash(n)),
            serde_json::to_string(&n).unwrap()
        );
    }

    // encoded, hash
    fn get_encoded_hash(&self, n: &Node) -> (Bytes, Bytes) {
        let encoded: Vec<u8> = bincode::serialize(&n).unwrap();
        let hash = self.database.compute_hash(&encoded.as_slice());
        (Bytes::from(encoded), Bytes::from(hash))
    }

    fn write_node(&self, n: &Node) -> Bytes {
        let (encoded, hash) = self.get_encoded_hash(n);
        self.database.write(&hash, &encoded[..]);
        hash
    }

    fn read_node(&self, key: &Bytes) -> Result<Node, ()> {
        let data = self.database.read(key).unwrap();
        let decoded: Node = bincode::deserialize(&data[..]).map_err(|_e| ())?;
        Ok(decoded)
    }

    pub fn do_put(&mut self, key: &Bytes, index: usize, value: &Bytes, parent: &mut Node) -> Bytes {
        let current = &key[index..index + 1];
        debug!(
            "do_put {}  oldone exist {}",
            hex::encode(&current),
            parent.children.contains_key(current)
        );
        let next_index = index + 1;
        let is_leaf = key.len() == next_index;

        if is_leaf {
            let mut newleaf = Node::default();
            // find old one, if exist
            if parent.children.contains_key(current) {
                debug!("use leaf");
                let hash_found = &parent.children[current];
                let node_found = self.read_node(&hash_found).unwrap();
                newleaf = node_found;
            } else {
                debug!("create leaf");
                // create
                newleaf.address = Bytes::from(current);
            }

            // update
            newleaf.value = value.clone();

            // update hash write
            let hash = self.write_node(&newleaf);
            debug!(
                "leaf {} key={}",
                serde_json::to_string(&newleaf).unwrap(),
                hex::encode(key)
            );

            parent.children.insert(Bytes::from(current), hash);
            // update hash, write
            let parenthash = self.write_node(&parent);
            return parenthash;
        } else {
            let mut newbranch = Node::default();

            // find old one, if exist
            if parent.children.contains_key(current) {
                debug!("use branch");
                let hash_found = &parent.children[current];
                let node_found = self.read_node(&hash_found).unwrap();
                newbranch = node_found;
            } else {
                debug!("create branch");
                newbranch.address = Bytes::from(current);
            }

            // update children
            let child_hash = self.do_put(&key, next_index, &value, &mut newbranch);
            // upsert
            debug!("insert child key={}", hex::encode(Bytes::from(current)));
            parent.children.insert(Bytes::from(current), child_hash);

            // update hash, write
            let hash = self.write_node(&parent);
            return hash;
        }
    }

    pub fn put(&mut self, key: &Bytes, value: &Bytes) {
        let mut root = self.root.clone();
        let _final_root = self.do_put(&key, 0, &value, &mut root);
        debug!("root children={:?}", root.children);
        self.root = root;
    }

    pub fn do_get(&self, key: &Bytes, index: usize, parent: &Node) -> Result<Bytes, ()> {
        let current = &key[index..index + 1];
        debug!(
            "do_get {}  oldone exist {}",
            hex::encode(&current),
            parent.children.contains_key(current)
        );
        let next_index = index + 1;
        let is_leaf = key.len() == next_index;
        if is_leaf {
            let hash = parent.children[current].clone();
            let found_node = self.read_node(&hash).unwrap();
            return Ok(found_node.value.clone());
        } else {
            if parent.children.contains_key(current) {
                let childnode = self.read_node(&parent.children[current]).unwrap();
                debug!("use branch");
                //let next = Bytes::from(&key[next_index..next_index + 1]);
                return self.do_get(&key, next_index, &childnode);
            } else {
                Err(())
            }
        }
    }

    pub fn get(&mut self, key: &Bytes) -> Result<Bytes, ()> {
        self.do_get(&key, 0, &self.root)
    }

    pub fn get_roothash(&self) -> Bytes {
        self.get_hash(&self.root)
    }

    pub fn show_roothash(&self) {
        println!("final root={}", hex::encode(&self.get_roothash()));
    }
}
