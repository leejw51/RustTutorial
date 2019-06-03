fn test1() 
{
    const n:usize= 10000000;
    let m:[i32;n]=[0;n]; 
}

struct Data {
    m: [i32;10000000]
}

fn main() 
{
   let m = Box::new(Data{m:[0; 10000000]});
   println!("OK2");
}
