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
fn test2() {
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

fn edit(m: &mut [i64; 3]) {
    m[0] = 100;
    let n = m;
}

fn main() {
    let mut a: [i64; 3] = [1, 2, 3];
    edit(&mut a);
    let mut b = &mut a;
    println!("b={:?}", b);
    b[2] = 150;
    println!("b={:?}", b);
    let mut c = &mut a;
    c[1] = 500;
    println!("c={:?}", c);
    println!("a={:?}", a);
}
