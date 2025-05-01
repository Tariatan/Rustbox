use std::time::Duration;

mod calculator;

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
}