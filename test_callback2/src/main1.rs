use std::sync::{Mutex,Arc};

#[derive(Default,Debug,Clone)]
struct Storage
{

}

#[derive(Default,Debug,Clone)]
struct DB
{

}

impl Storage 
{
    fn write(&self)
    {
        println!("write");
    }
}

fn process(storage:& Storage)
{
    storage.write();
}

#[derive(Debug,Default,Clone)]
struct Note {
    storage:  Option<Storage>,

}
fn main()
{
    println!("storage");
    let db=DB::default();
    let db2= db.clone();
    let mut note = Note{storage: Some(Storage::default())};
    note.storage.map(|s| s.write());
}