#![allow(unused)]

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::slice;

// Dereferencing raw pointers
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
fn calling_unsafe_function() {
    let mut v = vec![1, 2, 3, 4, 5];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &[1, 2, 3]);
    assert_eq!(b, &[4, 5, 6]);
}
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

fn access_static() {
    println!("value is: {HELLO_WORLD}");
}

static mut COUNTER: u32 = 0;
fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn read_write_static() {
    add_to_counter(3);
    /*unsafe*/ {
        // println!("COUNTER: {COUNTER}"); // shared references to mutable static is discouraged
    }
}

// Unsafe Traits
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// Advanced Traits
// Associated types
pub trait Iterator {
    type Item; // Associated type
    fn next(&mut self) -> Option<Self::Item>;
}

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

pub fn outlined_point() {
    let a = Point { x: 1, y: 2 };
    a.outline_print();
}
// Supertraits
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string(); // to_string comes from the Display supertrait
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {
    
}

// Using the Newtype pattern to implement External Traits
// Let's say we want to implement Display on Vec<T>, which the orphan rule prevents us from doing
// directly because the Display trait and the Vec<T> type are defined outside our crate.
// We can make a Wrapper struct that holds an instance of Vec<T>; then we can implement Display
// on Wrapper and use the Vec<T> value.
struct Wrapper(Vec<String>); // Tuple struct
impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(",")) // Vec<T> is the item at index 0 in the tuple
    }
}

pub fn wrapped_vector() {
    let w =  Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w = {w}");
}

// Type synonyms
type Kilometers = i32;
pub fn type_alias() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {}
    fn returns_long_type() -> Thunk {
        Box::new(|| println!("there"))
    }
}

// Never type '!' that never returns
// 'Diverging function' - The function bar returns never, since panic! terminates the program
fn bar() -> ! { 
    panic!("foo")
}

// Advanced Functions and Closures
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
pub fn do_math() {
    let answer = do_twice(add_one, 5); // (5 + 1) + (5 + 1)
    println!("The answer is: {}", answer);
}

// macro_rules!
#[macro_export]                // make this macro available whenever the crate is brought into scope
macro_rules! create_vec {
    ( $( $x:expr ),* ) => {    // match pattern arm
                               // ($) declares a variable in the macro system that will contain the Rust code
                               // matching the pattern
                               // $x:expr matches any Rust expression and gives the expression the name $x
                               // ',' indicates that a literal comma separator could optionally appear
                               // after the code that matches the code in $()
                               // '*' specifies that the pattern matches zero or more of whatever precedes the *
                               // in create_vec![1, 2, 3] the $x pattern matches 3 times with the 3 expressions 1, 2, and 3
        {
            let mut temp_vec = Vec::new();
            $(                          // block $()* is generated fore each part that matches $()
                temp_vec.push( $x);     // $x is replaced with each expression matched
            )*
            temp_vec                    // return result
        }
    };
}

pub fn macro_rules() {
    let v: Vec<i32> = create_vec![1, 2, 3];
}

macro_rules! hello_x {
    ("Charlie") => { println!("Hi Charlie") };
    ($s:ident) => { println!("Hello {}", $s) };
    ($s:literal) => { println!("Hello, {}", $s) };
    // expr catches literal and ident patterns
    ($s:expr) => { println!("Hello, {}", $s) };
}

pub fn greet() {
    hello_x!("Charlie");
    hello_x!("Bob");
    hello_x!(5);
}

macro_rules! set {
    ( $( $s:expr ),* ) => { HashSet::from([$($s),*]) }
}

fn test_set() {
    let a = set![1, 2, 3];
}
// Procedural macros require own crate