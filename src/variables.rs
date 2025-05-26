#[allow(dead_code)]
fn variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    another_function(5);
}

fn another_function(x: i32) {
    /*hello world*/
    println!("Another function. x is: {}", x);
    if x < 5 {
        println!("The value of x is: {}", x);
    }

    if x % 4 == 0 {}
    else if x % 4 == 1 {  }
    else if x % 4 == 2 {}
    else { println!("The value of x is: {}", x); }
}

