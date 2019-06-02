fn add(a: &String, b: &String) -> String {
    let mut ret = a.clone();
    ret.push_str(b);
    return ret;
}
fn print(a: &str) {
    println!("{}", a);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert!(add(&"1".to_string(), &"2".to_string()) == "12");
    }
}
fn main() {
    let a = String::from("apple");
    let b = String::from("pear");
    print(&a);
    let c = add(&a, &b);
    println!("{}", c);
}