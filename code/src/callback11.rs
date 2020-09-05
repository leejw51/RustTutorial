#[derive(Debug, Default)]
struct Note {
    info: String,
}

impl Note {
    pub fn run(&mut self) {
        println!("run");
    }
}
pub fn main() {
    let mut note = Note::default();
    note.run();
}
