pub fn first_word(string: &String) -> usize
{
    let bytes = string.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    string.len()
}