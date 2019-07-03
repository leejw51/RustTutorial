fn change(b: &mut [u8; 10]) {
    for i in 0..b.len() {
        b[i] = 2;
    }
}
fn test1() {
    let a = [0; 100];
    let mut b: [u8; 10] = [1, 2, 3, 4, 5, 6, 10, 20, 30, 15];
    {
        let mut c = b;
        for x in &c {
            println!("{}", x);
        }
    }
    println!("******************************************");
    change(&mut b);
    for x in &b {
        println!("{}", x);
    }
}
#[derive(Debug, Clone, Copy)]
struct Number {
    value: i64,
}
fn main() {
    let a = 100;
    {
        let mut c = a;
        c = 200;
    }
    println!("{}", a);

    let s1 = Number { value: 200 };
    let s2 = s1;
    println!("{:?}", s1);
}
