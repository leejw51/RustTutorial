struct Gun {
    is_loaded: bool,
    name: &'static str,
}

trait Item {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    fn load(&mut self);

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn defend(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn fire(&self) {
        println!("{} says {}", self.name(), self.defend());
    }
}

impl Gun {
    fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    fn shoot(&mut self) {
        if self.is_loaded() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already loaded...", self.name());
        } else {
            println!("{} needs loading!", self.name);

            self.is_loaded = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Item for Gun {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Gun {
        Gun {
            name: name,
            is_loaded: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn load(&mut self) {
        self.is_loaded = true;
    }
    fn defend(&self) -> &'static str {
        if self.is_loaded() {
            "fire!!"
        } else {
            "none"
        }
    }
    // Default trait methods can be overridden.
    fn fire(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} fire... {}", self.name, self.defend());
    }
}

fn shoot<T: Item>(item: &mut T) {
    item.fire();
}
fn main() {
    let mut gun: Gun = Item::new("laser");
    gun.load();
    shoot(&mut gun);
}
