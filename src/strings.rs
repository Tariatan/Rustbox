#![allow(unused)]

use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;
use std::{fs, thread};
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};


// Rust strings
// - No null (0) terminator
// - UTF-8 encoded
// - Immutable by default
// 
// String - heap allocated, growable UTF-8-encoded string. It's an owned type, responsible for cleaning its memory.
// &str - string slice, which is a view into a string or part of the string.
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
    
    // Raw string literal allow for special characters without needing to escape them
    let data = r#"initial "source" contents"#;
    
    // Byte-string literal generate a byte sequence, ex. for networking protocols
    let http_ok: &[u8; 17] = b"HTTP/1.1 200 OK\r\n";
    
    // Raw byte-string containing the PNG signature
    let png_signature = br#"\x89PNG\r\n\x1a\n"#;
    
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

// If needed to return &str type that will not be modified further.
fn boxed_str() {
    let my_string: String = String::from("This is a long string that I don't intend to modify further.");
    
    let my_boxed_str: Box<str> = my_string.into_boxed_str();
    
    // Now 'my_boxed_str' can be used just like a &str, but it's owned.
    println!("{}", my_boxed_str);
}

// Share ownership of an immutable string slice across multiple part of a program,
// without cloning the actual string slice data
fn shared_str() {
    let some_large_text: &'static str = "This is some large text that we want to work with.";
    let subsection: Rc<str> = Rc::from(&some_large_text[5..24]);
    
    let another_reference = Rc::clone(&subsection);
    let yet_another_reference = Rc::clone(&subsection);
    
    println!("First reference: {}", subsection);
    println!("Second reference: {}", another_reference);
    println!("Third reference: {}", yet_another_reference);
}


fn shared_atomic_str() {
    let text_string = String::from("This is some text that multiple threads will read.");
    let text_slice = &text_string[..];
    
    let shared_text: Arc<str> = Arc::from(text_slice);
    let mut handles = vec![];

    for _ in 0..3 {
        let text_ref = Arc::clone(&shared_text);
        let handle = thread::spawn(move || {
            println!("Thread is reading: {}", text_ref);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// Copy on write type is used to avoid new allocations when no string modification is necessary
fn sanitize(input: &str) -> Cow<str> {
    if input.contains("badword") {
        let sanitized: String = input.replace("badword", "*****");
        return Cow::Owned(sanitized);
    }
    
    // zero-cost operation as returning the borrowed string
    Cow::Borrowed(input)
}

// OS specific strings used in cross-platform code
fn os_string() -> std::io::Result<()> {
    let paths = fs::read_dir(".")?;
    
    for path in paths {
        match path { 
            Ok(entry) => {
                let os_string: OsString = entry.file_name();
                match os_string.into_string() {
                    Ok(string) => println!("Found a file: {}", string),
                    Err(os_string) => println!("Found a non-UTF-8 filename: {:?}", os_string),
                }
            }
            Err(_) => println!("Couldn't read the path."),
        }
    }
    
    Ok(())
}

fn path_string() -> std::io::Result<()> {
    // Use Path type to reference a directory
    let dir = Path::new("./");
    let content = read_file(dir, "example.txt")?;
    println!("Content: {}", content);
    
    Ok(())
}

fn read_file(dir: &Path, file_name: &str) -> std::io::Result<String> {
    // Construct full path to the file within the directory
    let mut full_path = PathBuf::from(dir);
    full_path.push(file_name);

    let mut file = File::open(full_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    Ok(content)
}

// CStr & CString are useful when interfacing a c-style code that expects a 0-terminated strings

fn getenv_extern(name: *const std::os::raw::c_char) -> *const std::os::raw::c_char {
    todo!()
}

use std::ffi::{CStr, CString};
fn getenv() {
    // Creating a 0-terminated c-style string
    let key = CString::new("Path").expect("CString::new failed");
    unsafe {
        let val = getenv_extern(key.as_ptr());
        if val.is_null() {
            let c_str = CStr::from_ptr(val);
            // Convert to a valid UTF-8 string
            let str_slice = c_str.to_str().unwrap();
            println!("{}", str_slice);
        } else { 
            println!("Nothing found");
        }
    }
}