/*
written by jongwhan lee
@2020.01.19

<= for the future =>
*/
use crate::database::Database;
use failure::Error;
use serde::Deserialize;
use serde::Serialize;

pub trait MerkletrieDatabase {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8>;
    fn write(&mut self, key: &[u8], data: &[u8]) -> Result<(), Error>;
    fn read(&self, key: &[u8]) -> Result<Vec<u8>, Error>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct SparseMerkletrie<T>
where
    T: MerkletrieDatabase,
{
    database: T,
    root: Node,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct Node {
    pub children: [Option<Vec<u8>>; 2],
    pub value: Vec<u8>,
}
impl<T> SparseMerkletrie<T>
where
    T: MerkletrieDatabase,
{
    pub fn new(database: T) -> Self {
        SparseMerkletrie {
            database,
            root: Node::default(),
        }
    }

    // encoded, hash
    fn get_encoded_hash(&self, n: &Node) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let encoded: Vec<u8> = bincode::serialize(&n)?;
        let hash = self.database.compute_hash(&encoded.as_slice());
        Ok((encoded.to_vec(), hash))
    }

    fn write_node(&mut self, n: &Node) -> Result<Vec<u8>, Error> {
        let (encoded, hash) = self.get_encoded_hash(n)?;
        self.database.write(&hash, &encoded[..])?;
        Ok(hash)
    }

    fn read_node(&self, key: &[u8]) -> Result<Node, Error> {
        let data = self.database.read(key)?;
        let decoded: Node = bincode::deserialize(&data[..])?;
        Ok(decoded)
    }

    pub fn put(&mut self, key: &[u8], value: &[u8], output: &mut String) {
        let mut root = self.root.clone();
        let roothash=self.do_put(key, value, 8 * key.len() as i32 - 1, output, &mut root).expect("ok");
        let (encoded,hash)=self.get_encoded_hash(&root).expect("compute hash");
        assert!(hash== roothash);
        self.root = root;
    }

    pub fn do_put(
        &mut self,
        key: &[u8],
        value: &[u8],
        index: i32,
        output: &mut String,
        parent: &mut Node,
    ) -> Result<Vec<u8>, Error> {
        let which_byte = key.len() as i32 - 1 - index / 8;
        let byte_value = key[which_byte as usize];
        let bit = index % 8;
        let flag_value = byte_value & 1 << bit;
        let flag = if flag_value > 0 { 1 } else { 0 };
        let is_leaf = 0 == index;

        output.push_str(&flag.to_string());
        println!(
            "index {} byte {} bit {}  byte_value {:02X} bit {}  flag {}",
            index, which_byte, index, byte_value, bit, flag
        );

        if is_leaf {
            let mut newleaf = match &parent.children[flag] {
                Some(hash_found) => self.read_node(&hash_found)?,
                None => {
                    let mut ret = Node::default();
                    // create
                    ret
                }
            };
            newleaf.value = value.to_vec();
            // update hash write
            let hash = self.write_node(&newleaf)?;

            parent.children[flag] = Some(hash);
            // update hash, write
            let parenthash = self.write_node(&parent)?;
            Ok(parenthash)
        } else {
            let mut newbranch = match &parent.children[flag] {
                Some(hash_found) => self.read_node(&hash_found)?,
                None => {
                    let mut ret = Node::default();
                    // create
                    ret
                }
            };
            // update children
            let child_hash = self.do_put(key, value, index - 1, output, &mut newbranch)?;
            parent.children[flag] = Some(child_hash.to_vec());
            let hash = self.write_node(&parent)?;
            Ok(hash)
        }
    }
}

pub fn sparse_main() {
    let m = vec![10, 20, 30];
    let index = 1;
    println!("{:?}", &m[index..=index]);
    let database = Database::new("./data");
    let mut a = SparseMerkletrie::new(database);
    let mut output = "".to_string();
    a.put(&vec![0xF1, 0xab, 0xc3], &vec![0, 1, 2], &mut output);
    println!("{}", output);

    println!("Hello, world!");
}
