trait Animal {
    fn make_sound(&self) -> String;
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> String {
        "meow".to_string()
    }
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> String {
        "woof".to_string()
    }
}

fn main () {
    let dog: Dog = Dog;
    let cat: Cat = Cat;
    let mut v: Vec<Box<Animal>> = Vec::new();
    v.push(Box::new(cat));
    v.push(Box::new(dog));
    for animal in v.iter() {
        println!("{}", animal.make_sound());
    }
}