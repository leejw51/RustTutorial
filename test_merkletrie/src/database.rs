use super::merkletrie::MerkletrieDatabase;
use blake2::{Blake2b, Digest};
use bytes::Bytes;
use rocksdb::DB;

pub struct Database {
    path: &'static str,
    db: DB,
}

impl MerkletrieDatabase for Database {
    fn compute_hash(&self, data: &[u8]) -> Bytes {
        let mut hasher = Blake2b::new();
        hasher.input(data);
        Bytes::from(&hasher.result()[..])
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
        let db = DB::open_default(path).unwrap();
        Database { path, db }
    }

    pub fn write(&self, key: &[u8], data: &[u8]) {
        self.db.put(key, data).unwrap();
    }

    pub fn read(&self, key: &[u8]) -> Option<Vec<u8>> {
        match self.db.get(key) {
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

#[cfg(test)]
pub mod test {
    use super::super::game::*;
    use super::*;
    use protobuf::*;
    #[test]
    fn test1() {
        let a = 1;
        assert!(a + 1 == 2);
    }

    #[test]
    fn test2() {
        let mut m = Chat::new();
        let src = "HELLO WORLD".to_string();
        m.set_query(src.clone());
        let encoded: Vec<u8> = m.write_to_bytes().unwrap();
        println!("ecoded={:?}", encoded.len());

        let decoded = parse_from_bytes::<Chat>(encoded.as_slice()).unwrap();
        println!("decoded={}", decoded.query);
        assert!(src == decoded.query);
    }
}
