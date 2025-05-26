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
