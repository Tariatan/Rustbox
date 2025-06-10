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
