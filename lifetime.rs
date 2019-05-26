fn compute<'a>(x:&'a i32, y: &'a i32) -> &'a i32 {
    if x< y  {
        return x
    }
    else {
        return y;
    }
}

fn main() {
    let x = compute(&100,&20);
    println!("{}", x);
}