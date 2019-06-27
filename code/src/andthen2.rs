fn add(a: String) -> Result<String, String> {
    Ok(format!("{}_apple", a))
}
fn add_pear(a: String) -> Result<String, String> {
    Ok(format!("{}_pear", a))
}
fn add_banana(a: String) -> Result<String, String> {
    Ok(format!("{}_banana", a))
}


fn main() {
    println!("OK");
    let a = String::from("computer");
    let b = add(a)
    .and_then(|x| {
        add_pear(x)    
    })
    .and_then(|x| {
        add_banana(x)
    });

    println!("b={:?}", b);
}
