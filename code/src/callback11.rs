type Mycallback = Box<dyn Fn(i64) -> i64>;

#[derive(Default)]
struct Note {
    info: String,
    mycallback: Option<Mycallback>,
}

impl Note {
    pub fn run(&mut self) {
        for id in 0..5 {
            let output=self.mycallback.as_ref().unwrap()(id);
        println!("run {} output {}",id, output);
        }
    }
}
pub fn main() {
    let  value=1000;
    let mut note = Note {
        info: "".into(),
        mycallback: None,
    };
    note.mycallback = Some(Box::new(move |a: i64| -> i64 { 
        a * value }));
    note.run();
}
