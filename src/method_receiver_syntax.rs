#[derive(Debug)]
pub struct CarRace {
    name: String,
    laps: Vec<i32>,
}

impl CarRace {
    // No receiver, a static method
    // Typically used to create constructors which are called new by convention.
    pub fn new(name: &str) -> Self {
        Self { name: String::from(name), laps: Vec::new() }
    }

    // Exclusive borrowed read-write access to self
    // Borrows the object from the caller using a unique and mutable reference.
    // The object can be used again afterwards.
    pub fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    // Borrows the object from the caller using a shared and immutable reference.
    // The object can be used again afterwards.
    pub fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self (covered later)
    // Takes ownership of the object and moves it away from the caller.
    // The method becomes the owner of the object.
    // The object will be dropped (deallocated) when the method returns, unless its ownership is explicitly transmitted.
    // Complete ownership does not automatically mean mutability.
    //
    // Also: mut self: same as above, but the method can mutate the object.
    pub fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}
