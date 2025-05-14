use std::time::Duration;
mod calculator;
mod method_receiver_syntax;
use method_receiver_syntax::*; // The Glob Operator
mod trait_impl;
mod loops;
mod ownership;
mod slices;
mod structures;
mod guessing_game;
mod variables;
mod enumerations;
use trait_impl::{Dog, Pet};
use crate::enumerations::{IpAddress, Message};
use crate::structures::Rectangle;

// Tuple Structs
struct Point(i32, i32);

fn sleep_for(secs: f32)
{
    if let Ok(duration) = Duration::try_from_secs_f32(secs)
    {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    }
    else
    {
        println!("failed to sleep for {secs} seconds");
    }
}

fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String>
{
    let Some(s) = maybe_string else { return Err(String::from("got None")); };

    let Some(first_byte_char) = s.chars().next() else { return Err(String::from("got empty string")); };

    let Some(digit) = first_byte_char.to_digit(16) else { return Err(String::from("not a hex digit")); };

    Ok(digit)
}
fn main()
{
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
    println!("{}", size_of::<usize>());
    println!("{}", size_of::<i32>());

    sleep_for(-0.5);

    let a: [i32; 10] = [10, 20, 30, 40, 50, 60, 72, 85, 93, 107];

    // Irrefutable pattern
    let [first, .., last] = a;
    println!("first: {first} last: {last}");

    // Slices
    let s: &[i32] = &a[2..6];
    println!("s: {s:?}");

    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop()
    {
        print!("{}", c);
    }
    println!();

    let s_to_hex = String::from("A");
    println!("{s_to_hex}: {:?}", hex_or_die_trying(Some(s_to_hex.clone())));

    use calculator::Expression;
    use calculator::Operation;
    // (5 * 3) + (10 / 2)
    let complex_expression = Expression::Op
    {
        op: Operation::Add,
        left: Box::new(Expression::Op
        {
            op: Operation::Mul,
            left: Box::new(Expression::Value(5)),
            right: Box::new(Expression::Value(3))
        }),
        right: Box::new(Expression::Op
        {
            op: Operation::Div,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(2))
        })
    };

    println!("{}", calculator::eval(complex_expression));

    let mut race = CarRace::new("Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();

    let fido = Dog {name: String::from("Fido"), age: 3};
    fido.greet();

    loops::break_returns_value();
    loops::loop_with_goto();
    loops::for_loop();

    ownership::ownership_with_string();
    ownership::ownership_with_methods();
    ownership::ownership_with_returning();
    ownership::ownership_reference();

    let mut slicing_string = String::from("Hello, world!");
    let word = slices::first_word(&slicing_string);
    slicing_string.clear();

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 45
    };

    dbg!(&rect1);
    println!("{}", rect1.area());
    println!("{}", rect1.can_hold(&Rectangle{ width: 30, height: 60 }));
    println!("{}", rect1.can_hold(&Rectangle::square(20)));

    let home = IpAddress::V4(127, 0, 0, 1);
    let home = IpAddress::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let x = Some(5);
    let six =     match x  {
        None => None,
        Some(i) => Some(i + 1),
        other => other,
    };

    let dice_roll = 9;
    match dice_roll {
        3 => sleep_for(0.3),
        2 => sleep_for(0.2),
        _ => (),                // empty tuple type -> means do nothing
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max { println!("config_max: {config_max:?}"); }
    if let None = config_max { println!("config_max: None"); }
}