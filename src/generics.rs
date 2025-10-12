#![allow(unused)]

pub fn duplicate<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}

#[allow(dead_code)]
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

#[allow(dead_code)]
enum Variant<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
struct Point<T>  {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
impl Point<f32>  {
    // Only instances of Point<f32> will have this method
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
fn print_area<T: Shape>(shape: &T) {
    println!("Area: {}", shape.area());
}

fn collect_shapes() {
    let rectangle = Rectangle {
        width: 30f64,
        height: 50f64,
    };
    print_area(&rectangle);
    
    let circle = Circle {
        radius: 30f64,
    };
    print_area(&circle);
    let _shapes: Vec<Box<dyn Shape>> = vec![Box::new(rectangle), Box::new(circle)];
}