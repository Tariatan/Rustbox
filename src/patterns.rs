// Mixing if let, else if, else if let, and else
#[allow(unused)]
pub fn conditional() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favorite, {}, as the background", color);
    }  else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        }  else {
            println!("Using orange as the background color");
        }
    }  else {
        println!("Using blue as the background color");
    }
    
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
    
    // Destucture a tuple
    let (_x, _y, _z) = (1, 2, 3);
    
    let point = (3, 4);
    print_coordinates(&point);
    
    let x = Some(5);
    let y = 10;
    match x { 
        Some(50) => println!("Got 50"),                 // Does not match, since x = Some(5)
        Some(y) => println!("Matched, y = {y}"),   // Match y = 5, since y - is a shadowed variable for outer y,
                                                        // matching any Some(value), Some(5) in this case
        _ => println!("Default case, x = {:?}", x),     // Would have matched if x = None
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);   // x = Some(5), y = 10, since y is an outer variable here
    
    // Multiple Patterns
    let x = 1;
    match x { 
        1 | 2 => println!("Got one or two"),
        1..5 => println!("one through five"),
        8 => println!("Got 8"),
        _ => println!("Default case, x = {:?}", x),
    }
    
    let c = 'c';
    match c { 
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("Default case, c = {:?}", c),
    }
    
    let p = Point { x: 0, y: 7 };
    // Destructuring a struct's fields into separate variables
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    
    // Destructuring and matching literal values in one pattern
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    
    // Destructuring enums
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg { 
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, value {}", h,  s, v);
        }
    }
    
    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    
    // Ignore match patterns
    
    // Match Some variants when we don't need to use the value inside the Some()
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    let origin = Point { x: 0, y: 7 };
    match origin {
        Point {x, ..} => println!("x is {x}"),
    }
    
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    
    // Extra Conditionals with Match Guards
    let num = Some(4);
    match num { 
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => println!("something else"),
    }
    
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // Match if x is (4, 5, or 6) and y is true
        _ => println!("no"),
    }
    
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Move {
            x: x_axis @ 0..=10,    // @ Bindings capture whatever value matched the range
                                        // while also testing that the value matched the range pattern
            y: _
        } => println!("On the x axis at {}", x_axis),
        Message::Move {
            x: _,
            y: 10..=20,
        } => println!("On the y axis at {}", y),
        _ => println!("Otherwise"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[allow(unused)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

pub fn print_coordinates(&(x, y): &(u32, u32)) {
    println!("Current location: ({}, {})", x, y);
}