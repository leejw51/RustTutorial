fn main(){
    let a= Some(100);
    {
        let b= a.unwrap();
        println!("b= {:?}",  b);
    }
    println!("a= {:?}", a);
}