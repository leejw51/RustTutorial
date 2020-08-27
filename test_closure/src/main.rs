type MyClosure= FnMut(i64)->bool;
struct Fruit {
    info: String,
    myclosure: Box<MyClosure>,
}

impl Fruit {
    pub fn  eat(&self) {
        println!("eat fruit");
    }
    pub fn run(&mut self) {
        for i in 0..10 {
            (*self.myclosure)(i);
        }
    }
}

fn main() {
    let mut count:i64 =10;
    let closure1= move | number: i64| -> bool {
        count+= number;
        println!("closure1 count {}", count);
        true
    };
    let mut a= Fruit {
        info:"0".into(),
        myclosure: Box::new(closure1),
    };
    a.run();
}
