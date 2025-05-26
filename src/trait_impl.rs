pub trait Animal {
    fn leg_count(&self) -> u32;
}

pub trait Pet: Animal {
    fn name(&self) -> String;
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

pub struct Dog {
    pub name: String,
    pub age: i8,
}
pub struct Cat {
    pub name: String,
    pub lives: i8,
}
impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Animal for Cat {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) -> String {
       format!("Miau! {} lives left!", self.lives)
    }
}
// Multiple impl blocks are allowed for a given type.
// Furthermore, impl blocks can even be spread across multiple modules/files.
impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) -> String {
        format!("Woof, my name is {}! I have {} legs! I'm {}", self.name(), self.leg_count(), self.age)
    }
}

// Uses generics and static dispatch.
pub fn generic(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// Uses type-erasure and dynamic dispatch.
// Represents two pointers:
//  -   to the data (an instance of a struct)
//  -   to a vtable.
pub fn dynamic(pet: &dyn Pet) {
    println!("Hello, who are you? {}", pet.talk());
}
