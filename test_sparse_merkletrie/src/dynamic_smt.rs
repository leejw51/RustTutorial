use super::database::{Database, MemoryDatabase};
use super::merkletrie_interface::MerkletrieDatabase;
use super::merkletrie_interface::MerkletrieInterface;
use bitvec::prelude::*;
use bitvec::prelude::*;
use bitvec::*;
use failure::Error;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};
type SMT_BYTES = BitVec<Msb0, u8>; // big endian
type SMT_SLICE = BitSlice<Msb0, u8>; // big endian

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
    pub children: BTreeMap<SMT_BYTES, Vec<u8>>,
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
        self.put(key, value);
        Ok(())
    }

    fn get(&mut self, key: &[u8]) -> Result<Vec<u8>, Error> {        
        self.get(key)
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

    pub fn show_root(&self) {
        let (encoded, hash) = self.get_encoded_hash(&self.root).expect("compute hash");
        println!("hash= {}", hex::encode(&hash));
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) {
        let mut root = self.root.clone();
        let bits = SMT_BYTES::from_slice(key);
        let roothash = self.do_put(&bits, value, &mut root).expect("ok");
        let (encoded, hash) = self.get_encoded_hash(&root).expect("compute hash");
        assert!(hash== roothash);
        self.root = root;
    }

    fn get_common(&self,src:&SMT_SLICE, src2: &SMT_SLICE)-> usize {
        //println!("get_common {:?} {:?}", src,src2);
        let mut n= src.len();
        if src2.len() < n {
            n= src2.len();
        }
        let i=0 as usize;
        for i in 0.. n {
            if src[i] !=src2[i]  {
                return i;
            }
        }
        n
    }
    pub fn do_put(
        &mut self,
        key_bits: &SMT_BYTES,
        value: &[u8],
        parent: &mut Node,
    ) -> Result<Vec<u8>, Error> {
        //println!("do put {:?} children={} value={}", key_bits, parent.children.len(), hex::encode(value));
        let mut i: usize= key_bits.len();
        let mut common: usize=0;
        let mut oldbranch : SMT_BYTES= SMT_BYTES::default();

        // update 
        if parent.children.contains_key(key_bits) {            
            let oldhash=parent.children[key_bits].clone();
            let mut oldnode= self.read_node(&oldhash)?;
            oldnode.value= value.to_vec();
            let hash = self.write_node(&oldnode)?;            
            parent.children.insert(key_bits.clone(), hash);
            let parenthash = self.write_node(&parent)?;
            return Ok(parenthash);
        }

       

        // find common key
        while i>=0 {
            let key= &key_bits[0..i];
            for (k, d) in &parent.children {
                common=self.get_common(&key, &k);
                if common>0 {
                    //println!("found common {}",common);
                    oldbranch= k.to_vec();
                    break;
                }
            }
            if common >0 {
                break;
            }
            //process            
            //println!("i={} {:?}", i,key);            
            if 0==i {
                break;
            }
            else {
                i=i-1;
            }
        }
       
        //0(includiing) ~ common(excluding)
        let mut is_leaf = 0==common;
        //println!("common={}", common);
        //println!("common = {} is_leaf = {}", common, is_leaf);
      if is_leaf {
            let mut index = key_bits.len();
            let mut new_leaf = Node::default();
            new_leaf.value = value.to_vec();
            let hash = self.write_node(&new_leaf)?;
            //println!("** add leaf key={:?} hash={}", &key_bits, hex::encode(&hash));
            parent.children.insert(key_bits.clone(), hash);
            let parenthash = self.write_node(&parent)?;
            Ok(parenthash)
        } else {
            let oldhash=parent.children[&oldbranch].clone();
            let oldnode= self.read_node(&oldhash)?;
            parent.children.remove(&oldbranch);
            assert!(!parent.children.contains_key(&oldbranch));
            
            //println!("split");
            // remove old branch
            let mut new_branchkey = &oldbranch[0..common];
            let mut new_branch = Node::default();            
            
            // make new children
            // oldnode
            let mut new_branchkey_a= &oldbranch[common..];
            let mut new_branchkey_b= &key_bits[common..];
            

            let new_branch_a_hash=  oldhash.clone();
            //println!("-------- before do_put");
            let new_branch_b_hash=self.do_put(&new_branchkey_b.to_vec(), 
            value, &mut new_branch)?;
            // link
            new_branch.children.insert(new_branchkey_a.to_vec(), new_branch_a_hash);
            let new_branch_hash = self.write_node(&new_branch)?;
            parent.children.insert(new_branchkey.to_vec(), new_branch_hash);
            //println!("children {}", parent.children.len());

            //println!("oldbranch= {:?}", new_branchkey);
           // println!("new branch a={:?}", new_branchkey_a);
            //println!("new branch b={:?}", new_branchkey_b);
            
            let hash = self.write_node(&parent)?;
            Ok(hash)
        }
    }

    pub fn get(&mut self, key: &[u8]) -> Result<Vec<u8>, Error> {
        let key_bits = SMT_BYTES::from_slice(key);
        self.do_get(&key_bits,  &self.root)
    }

    pub fn do_get(
        &self,
        key_bits: &SMT_BYTES,        
        parent: &Node,
    ) -> Result<Vec<u8>, Error> {
        //println!("do_get {:?} {}", key_bits, key_bits.len());
        if parent.children.contains_key(key_bits) {            
            let oldhash=parent.children[key_bits].clone();
            let mut oldnode= self.read_node(&oldhash)?;            
        //    println!("found key {:?} hash={} value={:?}",key_bits, hex::encode(oldhash), oldnode.value);
            return Ok(oldnode.value);


        }

        let mut i: usize= key_bits.len();
        let mut common: usize=0;
        let mut oldbranch : SMT_BYTES= SMT_BYTES::default();

        //println!("key={:?} children={}", key_bits, parent.children.len());
        // find common key
        while i>=0 {            
            let key= &key_bits[0..i];
            for (k, d) in &parent.children {                
    
                common=self.get_common(&key, &k);
                if common>0 {
                    //println!("found common {}",common);
                    oldbranch= k.to_vec();
                    break;
                }
            }
            if common >0 {
                break;
            }
            if 0==i {
                break;
            }
            else {
                i=i-1;
            }
        }
        
        if (0==common) {            
            return Err(format_err!("not found"));
        }
        let mut is_leaf = 0==common;
        //println!("is leaf {}", is_leaf);
        if is_leaf {
            assert!(parent.children.contains_key(&key_bits.clone()));
            let found= parent.children[&key_bits.clone()].clone();
            let node= self.read_node(&found)?;
            return Ok(node.value);
        }
        else {
            let oldhash=parent.children[&oldbranch].clone();
            let oldnode= self.read_node(&oldhash)?;

            let mut new_branchkey = &oldbranch[0..common];
            let mut new_branchkey_b= &key_bits[common..];
            return self.do_get(&new_branchkey_b.to_vec(), &oldnode);

        }

        Ok(vec![])
    }
}

pub fn dynamic_sparse_main() -> Result<(), failure::Error> {
    let database = MemoryDatabase::default();
    let mut smt = SparseMerkletrie::new(MemoryDatabase::default());
    //let database = Database::new("./data");
    //let mut smt = SparseMerkletrie::new(database.clone());
    let mut i: i32 = 0;
    let n = 1000;
    let now = Instant::now();
    for i in 0..n {
        let b = i as i32;
        let value = b.to_le_bytes();
        let key= database.compute_hash(&value);                
        smt.put(&key, &value);
    }
    println!("dynamic sparse merkletrie= {}", now.elapsed().as_millis());
    Ok(())
}

pub fn dynamic_sparse_main2() -> Result<(), failure::Error> {
    println!("dynamic_sparse_main");
    let database = MemoryDatabase::default();
    let mut smt = SparseMerkletrie::new(MemoryDatabase::default());
    smt.put(&hex::decode("f103")?, &hex::decode("0523")?);
    smt.show_root();
    smt.put(&hex::decode("f101")?, &hex::decode("01")?);
    smt.show_root();
   // smt.put(&hex::decode("f2")?, &hex::decode("01")?);
  //  smt.show_root();    
    let a=smt.get(&hex::decode("f101")?)?;
    println!("value={}", hex::encode(&a));
    
    Ok(())
}
