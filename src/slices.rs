pub fn first_word(string: &str) -> &str
{
    let bytes = string.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[..i];
        }
    }
    
    &string[..]
}

pub fn string_slices() {
    let s = String::from("Hello World");
    
    let _hello = &s[..5];
    let _world = &s[6..];
    let _hello_world = &s[..];
    
    // as long as method accepts &str it works with slices
    let _word = first_word(&s[..6]);
    let _word = first_word(&s[..]);
    
    // as well as with references to String
    let _word = first_word(&s);
    
    // string literals "are" string slices already
    let my_string_literal = "Hello World";
    let _word = first_word(my_string_literal);
}
