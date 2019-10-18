fn test1() {
     let a= [10,20,30];
    a.iter().for_each(|a| println!("{}", a));
}
fn main() {
    println!("OK");
    let a= [10,30,20,40,50];
    let b= a[0..3].to_vec();
    let c = b.as_slice();
    c.iter().for_each(|i| println!("{}",i));
}
