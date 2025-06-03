use std::collections::HashMap;
use std::hash::Hash;

#[allow(unused)]
pub fn hash_map() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".into(), 207);
    page_counts.insert("Grimms' Fairy Tales".into(), 751);
    page_counts.insert("Pride and Prejudice".into(), 303);

    let new_page_counts = HashMap::from([
        ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
        ("The Hunger Games".to_string(), 374),
    ]);

    for (key, value) in &page_counts {
        println!("{}: {}", key, value);
    }
    
    let new_book = String::from("New Book");
    page_counts.insert(new_book, 111);
    
    // println!("{new_book}"); // types with no Copy trait implemented are being moved into the map

    page_counts.extend(new_page_counts);
    
    // See if a book is in the hashmap and if not return an alternative value. 
    let pc = page_counts.get("Harry Potter and the Sorcerer's Stone").copied().unwrap_or(336);
    let pc1 = page_counts.get("Harry Potter and the Sorcerer's Stone").unwrap_or(&336);

    // Insert the alternative value in the hashmap if the book is not found.
    let pc2 = page_counts.entry("The Hunger Games".to_string()).or_insert(374);
    
    if !page_counts.contains_key("Les Misérables") {
        println!(
            "We know about {} books, but not Les Misérables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(&book.to_string()) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    // Use the .entry() method to insert a value if nothing is found.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    dbg!(page_counts);
}



#[allow(unused)]
struct Counter<T> {
    values: HashMap<T, u64>,
}

#[allow(unused)]
impl<T: Eq + Hash> Counter<T> {
    fn new() -> Self {
        Counter { values: HashMap::new() }
    }

    fn count(&mut self, value: T) {
        *self.values.entry(value).or_default() += 1;
    }

    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

#[allow(unused)]
pub fn counter_with_map() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}