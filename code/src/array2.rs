fn test1() {
    let a = vec![1000,20,300];
    for i in a.iter() { 
        println!("{:?}", i);
    }
}

fn main() 
{
    let a = [10,5,8];
    let b: Vec<i32> = a.into_iter().map(|x| {20*x}).collect();
    println!("{:?}", a);
}
