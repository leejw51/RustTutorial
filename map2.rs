fn test1() {
    let b: Result<i32, i32> = Ok(2);
    let a = b.map(|n| n + 1).map(|n| n * n).unwrap();
    println!("{}", a);
}

fn test2() {
    for a in (1..10).take(5) {
        println!("{}", a);
    }
}
fn test3() {
    let apple = (1..10).map(|n| n * n);
    for x in apple {
        println!("{}", x);
    }
}

fn test4() {
    let b: Result<i32, i32> = Ok(2);
    let a = b
        .and_then(|n| Ok(n + 1))
        .and_then(|n| Ok(n * 100))
        .map_err(|e| {
            println!("Error {}", e);
            return e - 1;
        }).map_err(|e| {
            println!("Error2 {}", e);
            return e - 1;
        }).and_then(|n: i32| Ok(n * n));
    match a {
        Ok(v) => println!("Success {}", v),
        Err(v) => println!("Fail {:?}", v),
    }
}

fn test5() {
    println!("test5");
    let res = (2..5).collect::<Vec<i32>>();
    let c: Vec<i32> = res
        .iter()
        .map(|n| {
            println!("Add {}", n);
            n - 1
        }).map(|n| {
            println!("Multiply {}", n);
            n * n
        }).map(|n| {
            println!("Multiply2  {}", n);
            n * 10
        }).collect();
    println!("{:?}", c);
}

fn test6() {
    for i in 1..10 {
        let a: Result<i32, i32> = Ok(i);
        let result = a
            .and_then(|n| {
                println!("Processing {}", n);
                Ok(n * 2)
            }).and_then(|n| if n < 5 { Ok(n) } else { Err(n) })
            .and_then(|n| Ok(n - 1))
            .and_then(|n: i32| Ok(n * n));
        println!("Result= {:?}", result);
    }
}

fn test7() {
    let a1: Vec<Result<i32, i32>> = vec![Ok(1), Ok(2), Ok(3)];
    for a in a1 {
        println!("{:?}", a);
        let result = a
            .and_then(|n| {
                println!("Processing {}", n);
                Ok(n * 2)
            }).and_then(|n| if n < 5 { Ok(n) } else { Err(n) })
            .map_err(|e| e * 22)
            .and_then(|n| Ok(n - 1))
            .and_then(|n: i32| Ok(n * n))
            .map_err(|e| e * 1000);
        println!("Result= {:?}", result);
    }
}

fn get_biggest<'a>(one: &'a i32, two: &'a i32) -> &'a i32 {
    if (one < two) {
        return two;
    } else {
        return one;
    }
}

fn main() {
    let s: i32 = 500;
    let s2: i32 = 1000;
    let a = get_biggest(&s, &s2);
    println!("information {}", a);
}
