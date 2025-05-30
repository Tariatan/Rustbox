#[allow(unused)]
pub fn strings() {
    let mut s1 = String::new();
    s1.push_str("Hello");

    // String::len returns the size of the String in bytes (which can be different from its length in characters).
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");

    // String::chars returns an iterator over the actual characters.
    println!("s3: len = {}, number of chars = {}", s3.len(), s3.chars().count());
}