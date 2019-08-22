fn main() {
    let a: Result<i32, i32> = Ok(200);

    let r = a.map(|a| "ok".to_string());
    println!("{:?}", r);

    let s = a.and_then(|a| Ok("ok".to_string()));
    println!("{:?}", s);

    let b = a.and_then(|g| {
        let mut ret = Ok("one hundred".to_string());
        if 100 == g {
            ret = Ok("one hundred".to_string());
        } else if 200 == g {
            ret = Ok("two hundred".to_string());
        }
        ret
    });
    println!("{:?}", b);
}
