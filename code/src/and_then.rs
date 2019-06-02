// and_then: active only in ok, output ok or err
// map: active only in ok, output ok
// map_err: active only in err, output err

fn process_number(i: i32) -> Result<i32, i32> {
    if i % 2 == 0 {
        return Ok(i / 2);
    } else {
        return Err(-1);
    }
}

fn main() {
    let result = process_number(3)
        .and_then(|n| { // can choose ok or err
            println!("and_then {:?}", n);
            //Err(n)
            Ok(n)
        })
        .map(|n: i32| { // output is ok
            println!("OK: {:?}", n);
            return n * n;
        })
        .map_err(|n| println!("FAIL: {:?}", n)) // output is err
        .map(|f| { 
            println!("Final {:?}", f);
            return f;
        });
    println!("*******************************");
    match result {
        Ok(v) => println!("OK {:?}", v),
        Err(v) => println!("FAIL {:?}", v),
    }
}
