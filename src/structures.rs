#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// Multiple impl blocks are allowed
impl Rectangle {
    // static method
    pub fn square(size: u32) -> Self {
        Self  { // Self is an alias for the struct type name 
            width: size,
            height: size,
        }
    }
}

pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,           // 'field init shorthand' is possible when parameters have
        email,              // the same name as struct fields
        sign_in_count: 1,
    }
}

pub fn struct_update_syntax(user: User, email: String) -> User {
    User {
        email: String::from("another@example.com"),
        ..user              // using struct update syntax to set as  new email value for a
                            // User instance but to use the rest of the values form 'user'
                            // !!! Must come last !!!
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
pub fn tuple_structs()
{
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}