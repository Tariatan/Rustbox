use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
        Red,
        Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        
        // The closure captures an immutable reference to the 'self' Inventory instance
        // and passes it with teh code we specify to the unwrap_or_else method.
        
        // If the Option<T> is the 'Some' variant,
        // unwrap_or_else returns the value from within the 'Some'.
        // 
        // If the Option<T> is 'None' variant, unwrap_or_else calls the closure
        // and returns the value returned by the closure.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        
        for color in &self.shirts {
            match color { 
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            } 
        }
        
        if num_red > num_blue {
            ShirtColor::Red
        } else { 
            ShirtColor::Blue
        }
    }
}

pub fn choose() {
    println!("-------------------------------------------");

    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
        ]
    };
    
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);
    
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
}

pub fn closure_examples() {
    // Type annotation is optional
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };
    
    // The type of the parameters and return type will be inferred by the first call of the closure.
    let example_closure = |x| x;
    
    let _s = example_closure(String::from("hello"));
    // let n = example_closure(1);     // Expected 'String' but found i32
    
    let list = vec![1, 2, 3];
    
    let only_borrows = || println!("From closure: {:?}", list);
    // Captures an immutable reference because it only needs an immutable reference to print the value.
    only_borrows();

    let mut list = vec![1, 2, 3];
    // Captures a mutable reference now.
    let mut borrows_mutably = || list.push(7);
    // println!("After calling closure: {:?}", list); // Cannot borrow 'list' as immutable
                                                      // because it is also borrowed as mutable. 
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    // 'move' forces the closure to take ownership of the values it uses in the environment
    // even though the body of the closure doesn't strictly need ownership.
    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
}

#[derive(Debug)]
#[allow(unused)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn sort_rectangles_by_key() {
    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
    ];
    
    // In this case the closure implements FnMut trait, since it is being called multiple times.
    list.sort_by_key(|r| r.width);
    println!("{:?}", list);

    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
    ];
    
    let mut num_sort_operations = 0;
    let _value = String::from("by key called");
    
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:?}, sorted in {num_sort_operations} operations.", list);
}