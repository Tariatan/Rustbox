use std::sync::Mutex;

pub fn ownership_with_string() {
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1;    // Value moved here
    println!("{}", s2);
    // println!("{}", s1);    // Value used after being moved
}

pub fn ownership_with_methods() {
    let s = String::from("Hello");
    
    takes_ownership(s); // value moves into the function and so is no longer valid here
    // println!("s: {}", s); // Value used after being moved

    let x = 5;
    
    makes_copy(x);  // trivial types are copied automatically
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The memory is freed.

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

pub fn ownership_with_returning() {
    let _s1 = gives_ownership();
    
    let s2 = String::from("hello");
    
    let _s3 = takes_and_gives_ownership(s2); // s2 is moved into method
    // println!("{}", s2); // Value used after being moved
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string                                         // moves out of the calling function
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string                                            // moves out of the calling function
}

pub fn ownership_reference() {
    let s1 = String::from("Hello");
    let _len = calculate_length(&s1);
    
    let mut s2 = String::from("Hello");
    let _r1 = &mut s2;   // first mutable borrow occurs here
    // let r2 = &mut s2;   // cannot borrow as mutable more than once at a time
    {
        let _r2 = &mut s2;   // multiple mutable references,
                             // just not simultaneous in the same scope
    }
    
    change(&mut s2);
    println!("{}", s2);
    
    let _r3 = &s1;          // Ok
    let _r4 = &s1;          // Ok
    // let r5 = &mut s1;    // cannot have a mutable reference while we have an
                            // immutable one to the same value
    
    let _r6 = &s2;
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn dangling_reference() {
    // let reference_to_nothing = dangle(); // Rust forbid
}

/*  Rust does not let pointing to an invalid object 
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/