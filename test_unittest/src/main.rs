#[derive(Clone, Debug)]
struct Apple {
    value: i32,
    kind: String,
}

trait Product {
    fn buy(&self, value: i32) -> bool {
        false
    }
}

trait Food {
    fn eat(&self) {
        println!("eat");
    }
}
impl Product for Apple {
    fn buy(&self, value: i32) -> bool {
        self.eat();
        if value > self.value {
            return true;
        } else {
            return false;
        }
    }
}
impl Food for Apple {
    fn eat(&self) {
        println!("eat apple {}", self.kind);
    }
}

fn main() {
    println!("Hello, world!");
    let a = Apple {
        value: 2000,
        kind: "mac".into(),
    };
    let b = a.buy(3000);
    println!("buy= {}", b);
    a.eat();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_apple() {
        assert!(false);
    }
}
