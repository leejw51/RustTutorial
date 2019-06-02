fn qsort(a: &Vec<i32>) -> Vec<i32> {
    if a.len() < 1 {
        return vec![];
    }

    let (x0, xs) = a.split_at(1);
    let x = x0[0];
    let smaller: Vec<i32> = xs.into_iter().filter(|y| y < x).collect();
    let bigger: Vec<i32> = xs.into_iter().filter(|y| y >= x).collect();

    let mut ret: Vec<i32> = vec![];
    let smaller2: Vec<i32> = qsort(&smaller);
    let bigger2: Vec<i32> = qsort(&bigger);
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
