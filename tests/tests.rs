pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
#[ignore]
fn it_doesnt_work() {
    panic!("Make this test fail!");
}

#[test]
fn greeting_contains_name() {
    let result = greeting("hello");
    assert_eq!(result.contains("hello"), true);
}

#[allow(unused)]
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}
#[test]
#[should_panic(expected = "Guess value must be between 1 and 100")]
fn guess_wrong() {
    Guess::new(200);
}

// Returning Result in test enables usage of the question mark operator in the body of the test and
// may be useful for tests that should fail if any operation within them returns an Err variant.
#[test]
fn it_also_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else { 
        Err(String::from("two plus two does not equal four"))
    }
}