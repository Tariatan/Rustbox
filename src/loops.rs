pub fn break_returns_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

pub fn loop_with_goto() {
    let mut counter = 0;
    'outer: loop {
        println!("counter = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'outer;
            }

            remaining -= 1;
        }
        counter += 1;
    }
    println!("End count = {counter}");
}

pub fn for_loop() {
    let mut a = [1, 2, 3, 4, 5];

    for element in a {
        println!("The value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}