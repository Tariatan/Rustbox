trait Animal {
    fn leg_count(&self) -> u32;
}

pub(crate) trait Pet: Animal {
    fn name(&self) -> String;
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

pub(crate) struct Dog {
    pub(crate) name: String,
    pub(crate) age: i8,
}

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

// Multiple impl blocks are allowed for a given type.
// Furthermore, impl blocks can even be spread across multiple modules/files.
impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}
