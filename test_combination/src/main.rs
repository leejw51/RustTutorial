use itertools::Itertools;
fn main() {
    let numbers = vec![10, 50, 8, 9, 200, 1000, 700];
    println!("numbers={:?}", numbers);
    let it: Vec<Vec<i32>> = numbers
        .into_iter()
        .combinations(3)
        .map(|mut combination| {
            combination.sort();
            combination
        })
        .collect();
    for i in it {
        println!("{:?}", i);
    }
}
