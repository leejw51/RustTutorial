#[derive(Debug, Default)]
struct Note {
    a: String,
}

#[derive(Debug, Default)]
struct Test {
    a: i32,
    note: Option<Note>,
}

fn take_value_from_option() {
    let mut m = Test {
        a: 10,
        note: Some(Note {
            a: "hello".to_string(),
        }),
    };

    let mynote: Note = m.note.take().unwrap();

    println!("{:?}", m);
    println!("note {:?}", mynote);
}

fn take_value_from_box() {
    let boxed = Box::new(Test {
        a: 10,
        note: Some(Note {
            a: "hello".to_string(),
        }),
    });

    let core = *boxed;
    println!("{:?}", core);
}

fn main() {
    take_value_from_option();
    take_value_from_box();
    
}
