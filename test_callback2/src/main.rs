
use std::sync::{Mutex,Arc};


trait Person {
    fn say_hello(&self);
  }


  struct PeopleZoo {
    people: Vec<Box<dyn Person>>
  }
  
  impl PeopleZoo {
    fn add_person(&mut self, person: Box<dyn Person>) {
      self.people.push(person);
    }
  
    
  }
  
trait Storage
{
    fn write(&self);

}


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




struct Note {
    storage:  Option<Box<dyn Storage>>,

}
fn main()
{
    println!("storage");
    let db=DB{};
  
     let mut note = Note{storage:Some(Box::new(db))};
    note.storage.map(|s| s.write());
}