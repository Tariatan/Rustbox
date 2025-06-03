use std::thread::yield_now;

#[allow(unused)]
pub fn vector_array() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    // Canonical macro to initialize a vector with elements.
    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    // Retain only the even elements.
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // Remove consecutive duplicates.
    v3.dedup();
    println!("{v3:?}");
    
    // let does_not_exist = &v3[100];   // Panics
    let does_not_exist = v3.get(100); // None, without panicking

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
    
    let first = &v3[0]; // Immutable borrow
    // v3.push(4);         // Mutable borrow forbidden
    let copy = first;   //  Immutable borrow

    // Immutable
    for i in &v3 {
        println!("{}", i);
    }
    
    // Mutable
    for i in &mut v3 {
        *i += 50;
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}