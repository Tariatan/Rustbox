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
    
    let data = "initial contents";
    
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    // println!("s1 = {}, s2 = {}", s1, s2); // value used after being moved
    
    let s1 = String::from("tic);");
    let s2 = String::from("tac);");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // uses references - no ownership taken
    println!("{s}");
    
    // let h = s[0]; // Rust strings don't support indexing
                     // as index into the string's byte
                     // will not always correlate to a valid Unicode scalar value
    
    for word in "hello wonderful world full of joys".split_whitespace() {
        println!("{}", word);
    }
}

pub fn surprise_inbound() {
    let s = String::from("SuRPRISE INbOUND");
    for c in s.chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase()) {
        println!("{}", c);
    }
}