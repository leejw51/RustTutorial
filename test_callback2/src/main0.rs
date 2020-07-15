fn check(number: i32, callback: Box< dyn Fn(i32) -> bool>) {
    for i in 0..number {
        if callback(i) {
            println!("{}", i);
        }
    }
}


#[derive(Debug,Clone)]
struct Test {
    value: i32,
}
fn main() {
  
    let mut test=Test{ value:0};
   
    check(10,  Box::new(move |a:i32| 
        { 
            let b=test.value;
         
            a % 2 == 0
        }));
}
