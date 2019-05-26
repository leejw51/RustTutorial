fn qsort(a: &Vec<i32>) -> Vec<i32> {
    if a.len() < 1 {
        return vec![];
    }

    let mut ret: Vec<i32> = vec![];
    let (x, xs) = a.split_first().unwrap();
    let smaller: Vec<i32> = xs.to_vec().into_iter().filter(|y| y < x).collect();
    let bigger: Vec<i32> = xs
        .to_vec()
        .into_iter()
        .filter(|y| y >= x)
        .collect();

    ret.append(&mut qsort(&smaller));
    ret.push(x.clone());
    ret.append(&mut qsort(&bigger));

    return ret;
}
fn main() {
    let a: Vec<i32> = vec![100, 50, 3, 7000, 200];
    let b = qsort(&a);
    println!("final={:?}", b);
}
