
fn add(a: i64) -> Result<i64, String>{
    Ok(a+1)
}
fn div(a: i64)-> Result<i64, String> {
    if (a==0) {
        return Err(String::from("zero error"));
    }
    Ok(100/a)
}
fn main() {
    let a = 0;
    let b= add(a)
    .map( |x| x*2)
    .and_then( |x| {
        let b = x-2;
        let c = div(b);
        c
    })
    .map( |x| {
        x*2
    });
    println!("{:?}", b);
}