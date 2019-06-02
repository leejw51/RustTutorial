fn qsort(a: &Vec<i32>) -> Vec<i32> {
    if a.len() < 1 {
        return vec![];
    }

    let mut xs: Vec<i32> = vec![];
    let x = a[0];
    let mut smaller: Vec<i32> = vec![];
    let mut bigger: Vec<i32> = vec![];
    for i in 1..a.len() {
        xs.push(a[i]);
    }
    for &y in &xs {
        if y < x {
            smaller.push(y);
        } else {
            bigger.push(y);
        }
    }

    let mut ret: Vec<i32> = vec![];
    let mut smaller2: Vec<i32> = qsort(&smaller);
    let mut bigger2: Vec<i32> = qsort(&bigger);
    ret.append(&mut smaller2);
    ret.push(x);
    ret.append(&mut bigger2);

    return ret;
}
fn main() {
    let a: Vec<i32> = vec![100, 50, 3, 7000, 200];
    let b = qsort(&a);
    println!("final={:?}", b);
}
