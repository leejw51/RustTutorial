fn skip_prefix<'a>(line: &'a str, prefix: &'a str) -> &'a str {
    line
}



fn main() {
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
