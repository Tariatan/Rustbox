use std::fmt::Display;

// The returned reference will be valid as long as both parameters are valid.
// This is the relationship between lifetimes of the parameters and the return type.
#[allow(unused)]
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b  
    }
}

#[allow(unused)]
pub fn test_lifetimes_ok() {
    let s1 = String::from("Hello");
    // All string literals have the 'static lifetime, which can be annotated as follows.
    // The test of this string is stored directly in the program's binary.
    let s2: &'static str = "I have a static lifetime.";
    // or same
    let s2 = "I have a static lifetime.";
    let _l = longest(s1.as_str(), s2);
}

#[allow(unused)]
pub fn test_lifetimes2_ok() {
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("xyz");
        let _l = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", _l);
    }
}

#[allow(unused)]
pub fn test_lifetimes3_failed() {
    let s1 = String::from("long string is long");
    let _l;
    {
        let s2 = String::from("xyz");
        _l = longest(s1.as_str(), "fail"/*s2.as_str()*/); // s2 does not live long enough
    }                                                        // s2 dropped here while still borrowed
    println!("Longest: {}", _l);
}

//////////////////////////////////////////////////////////////////////////
// A struct that holds a reference, requiring a lifetime annotation
#[allow(unused)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(unused)]
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[allow(unused)]
pub fn struct_lifetimes() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

// Lifetime elision rules (determine when no explicit annotations needed)
#[allow(unused)]
pub fn first_word(s: &str) -> &str {
    // Rule 1: compiler assigns a lifetime parameter to each parameter that's a reference
// fn first_word<'a>(s: &'a str) -> & str {

    // Rule 2: if there is exactly one input lifetime parameter,
    // the lifetime is assigned to all output lifetime parameters
// fn first_word<'a>(s: &'a str) -> &'a str {

    // Rule 3: if there are multiple input lifetime parameters,
    // but one of them is &self or &mut self because this is a method,
    // the lifetime of self is assigned to all output lifetime parameters.

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Generic type parameters, Trait bounds, and lifetimes together
#[allow(unused)]
pub fn longest_with_an_announcement<'a, T>
    (x: &'a str,
     y: &'a str,
     ann: T
    ) -> &'a str
where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else { 
        y
    }
}