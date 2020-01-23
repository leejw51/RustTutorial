/*
written by jongwhan lee
@2020.01.19

<= for the future =>
*/
use super::database::{Database, MemoryDatabase};
use failure::Error;
use serde::Deserialize;
use serde::Serialize;
use super::merkletrie_interface::MerkletrieInterface;
use super::merkletrie_interface::MerkletrieDatabase;
use std::time::{Duration, Instant};
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SparseMerkletrie<T>
where
    T: MerkletrieDatabase,
{
    database: T,
    root: Node,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Node {
    pub children: [Option<Vec<u8>>; 2],
    pub value: Vec<u8>,
}


impl<T> MerkletrieInterface for SparseMerkletrie<T>
where
    T: MerkletrieDatabase,
{
    fn load(&mut self, hash: &[u8]) -> Result<(), Error> {
        let node_found = self.read_node(&hash)?;
        self.root = node_found;
        Ok(())
    }

    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        let mut output="".to_string();
        self.put(key, value, &mut output);
        Ok(())
    }

    fn get(&mut self, key: &[u8]) -> Result<Vec<u8>, Error> {        
        let mut output="".to_string();
        self.get(key, &mut output)
    }

    fn get_roothash(&self) -> Result<Vec<u8>, Error> {
        self.get_hash(&self.root)
    }
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
     fn get_hash(&self, n: &Node) -> Result<Vec<u8>, Error> {
        Ok(self.get_encoded_hash(n)?.1)
    }

    // encoded, hash
    pub fn get_encoded_hash(&self, n: &Node) -> Result<(Vec<u8>, Vec<u8>), Error> {
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

    pub fn show_root(&self) 
    {
        let (encoded,hash)=self.get_encoded_hash(&self.root).expect("compute hash");
        //println!("hash= {}", hex::encode(&hash));      
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
        //println!(
         //   "index {} byte {} bit {}  byte_value {:02X} bit {}  flag {}",
          //  index, which_byte, index, byte_value, bit, flag
        //);

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

    pub fn get(&mut self, key: &[u8], output: &mut String) -> Result<Vec<u8>, Error> {
        self.do_get(&key, 8 * key.len() as i32 - 1,output, &self.root)
    }

    pub fn do_get(&self, key: &[u8], index: i32, output:&mut String, parent: &Node) -> Result<Vec<u8>, Error> {
        
        let which_byte = key.len() as i32 - 1 - index / 8;
        let byte_value = key[which_byte as usize];
        let bit = index % 8;
        let flag_value = byte_value & 1 << bit;
        let flag = if flag_value > 0 { 1 } else { 0 };
        let is_leaf = 0 == index;
        if is_leaf {            
            let hash = parent.children[flag].as_ref().ok_or_else(|| format_err!("key doesn't exist"))?.clone();
            let found_node = self.read_node(&hash)?;
            Ok(found_node.value)            
        } else if let Some(hash)=parent.children[flag].as_ref() {
            let childnode = self.read_node(&hash)?;
            self.do_get(&key, index-1, output,&childnode)
        } else {
            Err(format_err!("key doesn't exist"))
        }
    }
}




pub fn sparse_main2()->Result<(),failure::Error> {
    let database = Database::new("./data");
    let mut a = SparseMerkletrie::new(database);
    let mut output = "".to_string();
    let key=hex::decode("f1ab01")?;
    a.put(&key, & hex::decode("11a2f3")?, &mut output);
    println!("{}", output);
    let mut output2 = "".to_string();
    let read=a.get(&key, &mut output2)?;
    println!("{}", output);
    println!("read= {}", hex::encode(&read));
    Ok(())
}


pub fn sparse_main()->Result<(),failure::Error> {
    let database= MemoryDatabase::default();
    let mut smt = SparseMerkletrie::new(MemoryDatabase::default());
    //let database = Database::new("./data");
    //let mut smt = SparseMerkletrie::new(database.clone());
    let mut i:i32=0;
    let n= 1000;
    let now = Instant::now();
    for i in 0..n {
        let b = i as i32;
        let value= b.to_le_bytes();
        let key= database.compute_hash(&value);
    
        let mut output = "".to_string();
       // println!("{} {}", i, hex::encode(&key));
        smt.put(&key, &value, &mut output);
    }
    println!("sparse merkletrie= {}", now.elapsed().as_millis());
    Ok(())
}