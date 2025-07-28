use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

// Reasons to choose Box<T>, Rc<T>, or RefCell<T>:
// - Rc<T> enables multiple owners of the same data;
//   Box<T> and RefCell<T> have single owners.
// - Box<T> allows immutable or mutable borrows checked at compile time;
//   Rc<T> allows only immutable borrows checked at compile time;
//   RefCell<T> allows immutable or mutable borrows checked at runtime.
// - Because RefCell<T> allows mutable borrows checked at runtime,
//   you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

// Mutating the value inside an immutable value is the interior mutability pattern.

// Boxes allow to store data on the heap rather than the stack.
// What remains on the stack is the pointer to the heap data.
// Used in these situations:
// - When you have a type whose size can't be known at compile time, and you
//   want to use a value of that type in a context that requires an exact size.
// - When you have a large amount of data, and you want to transfer ownership
//   but ensure the data won't be copied when you do so.
// - When you want to own a value, and you care only that it's a type that
//   implements a particular trait rather than being of a specific type.

#[allow(unused)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn using_box() {
    let b =  MyBox::new(5);
    println!("b = {b}");
    drop(b);    // std::mem:drop forces a value to be cleaned up early.
    // println!("b = {b}"); // value used after being moved
    
    let m = MyBox::new(String::from("Rust"));
    // Calling hello with a reference to a MyBox value works because of deref coercion:
    // - Rust can turn &MyBox<String> into &String by calling deref,
    // - Rust calls deref again on &String into &str, which now matches the function signature.
    // The number of deref calls is resolved at compile time, so no runtime penalty.
    hello(&m);
    // Without deref coercion the call looks like this:
    hello(&(*m)[..]);
    
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

pub fn hello(name: &str) {
    println!("Hello, {name}!");
}

// A tuple struct
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0     // return the first value in a tuple struct.
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox!");
    }
}

impl<T: Display> Display for MyBox<T> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("{}", self.0);
        Ok(())
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//Rc<T>, the Reference Counted Smart Pointer
#[allow(unused)]
enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

use RcList::{RcCons, RcNil};

pub fn using_rc() {
    let a = Rc::new(RcCons(1,Rc::new(RcCons(2,Rc::new(RcCons(3,Rc::new(RcNil)))))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Rc::clone doesn't make a deep copy of all the data, but it only increments the reference count.
    let _b = RcCons(3, Rc::clone(&a)); // a.clone() is allowed, but Rc::clone() is Rust's convention.
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// RefCell<T> and the Interior Mutability Pattern

#[derive(Debug)]
#[allow(unused)]
enum RefList {
    RefCons(Rc<RefCell<i32>>, Rc<RefList>),
    RefNil,
}


#[derive(Debug)]
#[allow(unused)]
enum LeakingList {
    LeakingCons(i32, RefCell<Rc<LeakingList>>),
    LeakingNil,
}

impl LeakingList {
    fn tail(&self) -> Option<&RefCell<Rc<LeakingList>>> {
        match self {
            LeakingCons(_, item) => Some(item),
            LeakingNil => None,
        }
    }
}

use RefList::{RefCons, RefNil};
use LeakingList::{LeakingCons, LeakingNil};

pub fn using_ref_cell() {
    let _x = 5;
    // let y = &mut x; // not allowed to have mutable reference of immutable variable
    
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(RefCons(Rc::clone(&value), Rc::new(RefNil)));
    let b = RefCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RefCons(Rc::new(RefCell::new(7)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    
    
    // Simulate memory leak by creating a reference cycle of lists a and b pointing at each other.
    let a = Rc::new(LeakingCons(5, RefCell::new(Rc::new(LeakingNil))));
    println!("a initial rc count = {:?}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    
    let b = Rc::new(LeakingCons(7, RefCell::new(Rc::clone(&a))));
    println!("a rc count after creating b = {}", Rc::strong_count(&a));
    
    println!("b initial rc count = {:?}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    
    // this will overflow the stack
    // println!("a next item = {:?}", a.tail());
    
    // Rust drops 'b', decreasing the reference count of b from 2 to 1.
    // Then Rust drops 'a', decreasing the reference count of a  from 2 to 1 as well.
    // The memory will remain uncollected forever since other instances still refer to it.
}

#[allow(unused)]
pub trait Messenger {
    // Takes immutable reference to self
    fn send(&self, msg: &str);
}
#[allow(unused)]
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

#[allow(unused)]
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {messenger, value: 0, max}
    }
    
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }  else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You're at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You're at 75% of your quota!");
        }
    }
}


// Test using RefCell<T> to mutate an inner value while the outer value is considered immutable.
// RefCell<T> allows many immutable borrows or one mutable borrow at any point in time,
// otherwise the implementation of RefCell<T> will panic at runtime.
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        pub fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut(); // already borrowed
            
            one_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        // using borrow on the RefCell<T> to get an immutable reference
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1)
    }
}

// Weak references, Weak<T>, don't express an ownership relationship, and their count doesn't affect
// when a Rc<T> instance is cleaned up.
#[derive(Debug)]
#[allow(unused)]
struct Node {
    value: i32,
    // Can't be a Rc<T> because that would create a reference cycle with leaf.parent
    // pointing to branch and branch.children pointing to leaf,
    // which would cause their strong_count values to never be 0.
    // If a parent node is dropped, its child nodes should be dropped as well.
    // However, a child should not own its parent:
    // dropping a child node should not drop parent.
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// Using Weak<> to avoid reference cycle.
pub fn weak_reference() {
    let leaf = Rc::new(
        Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![])});
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(
            Node {
                value: 2,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

        // Rc::downgrade creates a Weak<Node> reference to branch from the Rc<Node> in branch.
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}