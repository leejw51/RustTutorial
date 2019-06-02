fn main() 
{
    let mut src:Result<String,String> = Ok(String::from("apple")) ;
//    x= Err("pear");
    println!("src={:?}", src);
    let value=
    src.map( |x| {
        format!("1_{}", x)
    }).
    map(|x| {
        format!("2_{}",x)
    }).
    and_then(|x| {
        println!("and_then {:?}",x);
        if x=="apple" {
            Ok(format!("+_{}", x))
        }
        else {
            Err(format!("-_{}", x))
        }
    }).
    map_err(|x| {
        format!("a_{}", x)
    }).
    map_err(|x| {
        format!("a_{}", x)
    });
    
    
    println!("value={:?}", value);
}
