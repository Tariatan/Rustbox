#![allow(unused)]

use std::collections::HashSet;
use std::error::Error;
use std::num::ParseIntError;
use std::sync::atomic::AtomicUsize;
use std::sync::TryLockError::Poisoned;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use log::log;
use crate::traits::Ex;

pub struct Image;

impl Image {
    pub fn make_thumbnail(&self) -> Self {
        Self
    }
}

pub fn make_thumbnails_simple(images: &[Image]) -> Vec<Image> {
    let mut i = 0;
    let len = images.len();
    let mut output = Vec::with_capacity(len);
    while i < len {
        output.push(images[i].make_thumbnail());
        i += 1;
    }

    output
}

// Improved
pub fn make_thumbnails(images: &[Image]) -> Vec<Image> {
    images
        .iter()
        .map(|image| image.make_thumbnail())
        .collect()
}

// Multithreaded with Rayon
pub fn make_thumbnails_multithreaded(images: &[Image]) -> Vec<Image> {
    images
        .par_iter()
        .map(|image| image.make_thumbnail())
        .collect()
}

pub fn count_thumbnails_race_conditioned(images: &[Image]) -> Vec<Image> {
    let mut counter = 0;
    let vec = images
        .par_iter()
        .map(|image| {
            // counter += 1;    // race condition
            image.make_thumbnail()
        })
        .collect();
    println!("{}", counter);
    vec
}

pub fn count_thumbnails_safe(images: &[Image]) -> Vec<Image> {
    let counter = AtomicUsize::new(0);
    let vec = images
        .par_iter()
    .map(|image| {
        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        image.make_thumbnail()
    })
        .collect();
    println!("{}", counter.load(std::sync::atomic::Ordering::SeqCst));
    
    vec
}

pub fn filter_items() {
    let items = (1..10).into_iter();
    let evens: Vec<_> = items
        .filter(|&i| i % 2 == 0)
        .collect();
    
    // vec![2, 4, 6, 8]
}

pub fn fold_items() {
    let items = (1..10).into_iter();
    items.fold(0, |sum, item| sum + item);
    
    // 1 + 2 + 3 + 4 +... = 45 
}

pub fn fancy_print(s: String) {
    print!("value: {}", s);
}

pub fn fancy_print_better(s: &str) {
    print!("value: {}", s);
}

#[derive(Debug, Default, Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
}
fn count_differences() {
    let points: Vec<Coordinates> = vec!
    [
        Coordinates { x: 1, y: 1 },
        Coordinates { x: 2, y: 2 },
    ];
    
    let mut differences = Vec::new();
    for i in 1..points.len() {
        let current = &points[i];
        let previous = &points[i - 1];
        differences.push(
            Coordinates {
                x: current.x - previous.x, 
                y: current.y - previous.y, 
            },
        );
    }
}
// In case adjacent elements are processed => use 'window'
fn count_differences_better() -> Vec<Coordinates> {
    let points: Vec<Coordinates> = vec!
    [
        Coordinates { x: 1, y: 1 },
        Coordinates { x: 2, y: 2 },
    ];

    points
        .windows(2)
        .map(|window| Coordinates{
            x: window[1].x - window[0].x,
            y: window[1].y - window[0].y,
        })
        .collect()
}

fn error_checking(a: &str, b:  &str) -> Result<i32, ParseIntError> {
    let a = a.parse::<i32>();
    if let Err(e) = a {
        return Err(e);
    }

    let b = b.parse::<i32>();
    if let Err(e) = b {
        return Err(e);
    }
    Ok(a.unwrap() + b.unwrap())
}

fn error_propagation(a: &str, b:  &str) -> Result<i32, ParseIntError> {
    let a = a.parse::<i32>()?;
    let b = b.parse::<i32>()?;
    Ok(a + b)
}

// Function might better return Result if there are multiple reasons to fail
fn might_return_result_instead(x: i32) -> Option<i32> {
    if x % 2 == 0 { return None; }
    if x % 3 == 0 { return None; }
    
    Some(x * 3)
}

// Function might better return Option if there is only one reason to fail
struct NotFound;
fn db_lookup() -> Result<i32, NotFound> {
    todo!() // allows empty function implementation even when the function must return value
}

fn fallible(_: u8) -> Result<String, String> {
    todo!()
}

struct W(String, String);
impl W {
    fn new(x: u8, y: u8) -> Result<Self, String> {
        // Possible but cumbersome
        let possible_return_value = fallible(x)
            .and_then(|x| {
            fallible(y)
                .map(|y| Self(x, y))
        });
        
        // Cleaner
        Ok(Self(fallible(x)?, fallible(y)?))
    }
}

struct V(Vec<String>);
impl V {
    fn new(v: &[u8]) -> Result<Self, String> {
        let v = v.iter()
            .map(|&x| fallible(x))
            .collect::<Result<Vec<_>, _>>();
        Ok(Self(v?))
    }
}

#[derive(Debug)]
enum GrandparentError {
    NoParent,
    NoGrandparent,
}

#[derive(Debug)]
struct Node;
impl Node {
    fn parent(&self) -> Option<&Node> { 
        todo!()
    }
    
    fn grandparent(n: &Node) -> Result<&Node, GrandparentError> {
        Ok(
            n.parent().ok_or(GrandparentError::NoParent)?
                .parent().ok_or(GrandparentError::NoGrandparent)?
        )
    }
}

// using iterator zero-cost abstraction to find the maximum element
fn max_element() {
    let numbers = vec![1, 2, 3, 4, 5];
    let max = numbers.iter().max();
    match max {
        Some(&max) => println!("{}", max),
        None => println!("No element"),
    }
}

fn vec_to_set() {
    let numbers = vec![1, 2, 3, 4, 5];
    numbers.clone().into_iter().for_each(|x| println!("{}", x));
    let set: HashSet<_> = numbers.into_iter().collect();
}

fn branching() {
    let result = if let Ok(db_lookup) = db_lookup() {
        println!("db_lookup: {}", db_lookup);
    } else { 
        println!("db_lookup: not found");
    };
}



