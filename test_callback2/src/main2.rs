use std::sync::{Mutex,Arc};

trait Storage
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
struct Note<'a> {
    storage:  &'a Storage ,

}
fn main()
{
    println!("storage");
    let db=DB::default();
    let db2= db.clone();
    let mut note = Note{storage:&db2};
  //  note.storage.map(|s| s.write());
}