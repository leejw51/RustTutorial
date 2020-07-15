use std::sync::{Mutex,Arc};

trait Storage: Clone + Sized
{
    fn write(&self);

}

#[derive(Default,Debug,Clone)]
struct DB
{

}

impl Storage for DB
{
    fn write(&self)
    {
        println!("write");
    }
}



#[derive(Clone)]
struct Note {
    storage:  Arc<Mutex<dyn Storage>> ,

}
fn main()
{
    println!("storage");
    let db=DB::default();
    let db2= db.clone();

    let mut note = Note{storage:Arc::new(Mutex::new(db2))};
  //  note.storage.map(|s| s.write());
}