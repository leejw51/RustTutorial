fn skip_prefix<'a>(line: &'a str, prefix: &'a str) -> &'a str {
    line
}


struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn test2() {
    let y = &5; // This is the same as `let _y = 5; let y = &_y;`.
    let f = Foo { x: y };

    println!("x is: {}", f.x());
}



fn test1() {
    println!("Hello, world!");
    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
        v = skip_prefix(line, p.as_str());  //  |
        println!("{}",v);
    }
}


fn main() {
    test2();
}