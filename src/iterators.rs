pub fn iterators_example() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
    
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("Sum: {:?}", total);
    
    // v1_iter.next(); // Value used after being moved
    
    // iterator and iterator adapters are lazy, and must be consumed.
    // v1.iter();
    
    let _v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter takes ownereship of the vector
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub fn filters_by_size() {
    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker"),},
        Shoe {size: 13, style: String::from("sandal"),},
        Shoe {size: 10, style: String::from("boot"),},
    ];
    let in_my_size = shoes_in_size(shoes, 10);
    println!("{:?}", in_my_size);
}