trait Fruit {
    fn eat(&self) {
        println!("eat fruie");
    }
}

struct Apple {}
impl Fruit for Apple {
    fn eat(&self) {
        println!("eat apple");
    }
}

struct Pear {}
impl Fruit for Pear {
    fn eat(&self) {
        println!("eat pear");
    }
}

#[derive(Default)]
struct Program {
    apple: Option<Apple>,
    pear: Option<Pear>,
}
impl Program {
    pub fn new() -> Self {
        let apple = Apple {};
        Program {
            apple: Some(apple),
            pear: None,
        }
    }
    pub fn getCurrent(&mut self) -> &(Fruit + 'static) {
        self.apple.as_mut().unwrap()
    }
    pub fn process(&mut self) {
        self.getCurrent().eat();
    }
}

fn main() {
    let mut a = Program::new();
    a.process();

    println!("go");
}
