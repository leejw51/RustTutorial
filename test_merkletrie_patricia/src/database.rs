use super::patricia_merkletrie::MerkletrieDatabase;
use blake2::{Blake2b, Digest};
use rocksdb::DB;
use std::sync::Arc;
use std::sync::Mutex;

type DBShared = Arc<Mutex<DB>>;
#[derive(Clone, Debug)]
pub struct Database {
    path: &'static str,
    db: DBShared,
}

impl MerkletrieDatabase for Database {
    fn compute_hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Blake2b::new();
        hasher.input(data);
        hasher.result()[..].to_vec()
    }
    fn write(&self, key: &[u8], data: &[u8]) {
        self.write(key, data);
    }
    fn read(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.read(key)
    }
}
impl Database {
    pub fn new() -> Database {
        let path = "mydb.dat";
        let dbshared = Arc::new(Mutex::new(DB::open_default(path).unwrap()));
        Database { path, db: dbshared }
    }

    pub fn write(&self, key: &[u8], data: &[u8]) {
        let db = self.db.lock().unwrap();
        db.put(key, data).unwrap();
    }

    pub fn read(&self, key: &[u8]) -> Option<Vec<u8>> {
        let db = self.db.lock().unwrap();
        match db.get(key) {
            Ok(Some(value)) => Some(value.to_vec()),
            Ok(None) => None,
            Err(_e) => None,
        }
    }

    // for debug
    pub fn read_string(&self, key: &[u8]) -> String {
        let value = self.read(key).unwrap();
        String::from_utf8(value).unwrap()
    }

    pub fn initialize(&mut self) {
        let _path = self.path;
        self.write(b"apple", b"computer");
        println!("read {}", self.read_string(b"apple"));
    }
}
