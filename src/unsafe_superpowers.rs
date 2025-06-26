use std::ops::Add;
use std::slice;

// Dereferencing raw pointers
#[allow(unused)]
pub fn dereferencing_raw_pointers() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// Calling an unsafe function
#[allow(unused)]
fn calling_unsafe_function() {
    let mut v = vec![1, 2, 3, 4, 5];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &[1, 2, 3]);
    assert_eq!(b, &[4, 5, 6]);
}
#[allow(unused)]
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
#[allow(unused)]
unsafe fn dangerous() {
    unsafe {
        if true {
            dangerous()
        }
    }
}

// Using extern functions
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

#[allow(unused)]
fn calling_extern_function() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Hello from Rust!");
}

// Accessing or modifying a mutable static variable (global variables in Rust)
static HELLO_WORLD: &str = "Hello, world!";

#[allow(unused)]
fn access_static() {
    println!("value is: {HELLO_WORLD}");
}

#[allow(unused)]
static mut COUNTER: u32 = 0;
#[allow(unused)]
fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
#[allow(unused)]
fn read_write_static() {
    add_to_counter(3);
    /*unsafe*/ {
        // println!("COUNTER: {COUNTER}"); // shared references to mutable static is discouraged
    }
}

// Unsafe Traits
#[allow(unused)]
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// Advanced Traits
// Associated types
#[allow(unused)]
pub trait Iterator {
    type Item; // Associated type
    fn next(&mut self) -> Option<Self::Item>;
}

#[allow(unused)]
struct Counter {
    items: Vec<u32>,
    index: usize,
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.items.get(self.index).map(|x| *x)
    }
}

// Operator overloading
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[allow(unused)]
fn operator_overloading() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 3, y: 4 };
    let _c = a + b;
}

struct Millimeters(u32);
struct Meters(u32);

// newtype pattern
// By default Rhs=Self, however newtype pattern implies new type: Rhs=Meters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Methods with the same name
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub fn disambiguator() {
    let person = Human;
    person.fly();         // "*waving arms furiously*"
    Human::fly(&person);  // "*waving arms furiously*"
    Pilot::fly(&person);  // "This is your captain speaking."
    Wizard::fly(&person); // "Up!"
    <Human as Wizard>::fly(&person);    // "Up!"
}