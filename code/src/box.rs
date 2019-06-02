fn main() {
    let a= 100;
    let b = Box::new(200);
    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",*b);
    println!("{:?}",&b);
    assert!(a==100);
    assert!(*b==200);
}
