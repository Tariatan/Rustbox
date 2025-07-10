#![allow(unused)]
use std::fmt::Error;

pub fn numeric_conversion() {
    let x: i32 = 42;
    let y: i8 = x as i8; // can overflow

    let y = i8::try_from(x).ok().ok_or("Number is too big to be used here");


}

/////////////////////////////////////////////////
// Bounded types
/////////////////////////////////////////////////
// DON'T: Use raw numeric types for domain values
// struct Measurement {
//     distance: f64,  // Could be negative!
// }
struct Measurement {
    distance: Distance,
}

// DO: Create bounded types
#[derive(Debug, Clone, Copy)]
struct Distance(f64);

impl Distance {
    pub fn new(value: f64) -> Result<Self, Error> {
        if value < 0.0 || !value.is_finite() {
            return Err(Error);
        }
        Ok(Distance(value))
    }
}

/////////////////////////////////////////////////
// DON'T: Use primitive types for usernames
/////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Username(String);

#[derive(Debug)]
enum UsernameError {
    Empty,
    TooLong,
    InvalidCharacters,
}

impl Username {
    pub fn new(name: &str) -> Result<Self, UsernameError> {
        // Check for empty username
        if name.is_empty() {
            return Err(UsernameError::Empty);
        }

        // Check length (for example, max 30 characters)
        if name.len() > 30 {
            return Err(UsernameError::TooLong);
        }

        // Only allow alphanumeric characters and underscores
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(UsernameError::InvalidCharacters);
        }

        Ok(Username(name.to_string()))
    }

    /// Allow getting a reference to the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn authenticate_user(username: Username) {
    // We know this is always a valid username!
    // No empty strings, no emojis, no spaces, etc.
}


/////////////////////////////////////////////////
// Make Invalid States Unrepresentable
/////////////////////////////////////////////////
// DON'T: Allow invalid combinations
// struct Configuration {
//     port: u16,
//     host: String,
//     ssl: bool,                   // Must never be true if cert is None
//     ssl_cert: Option<String>,
// }

// Define the possible states for the connection
enum ConnectionSecurity {
    Insecure,
    // We can't have an SSL connection
    // without a certificate!
    Ssl { cert_path: String },
}

struct Configuration {
    port: u16,
    host: String,
    // Now we can't have an invalid state!
    // Either we have an SSL connection with a certificate
    // or we don't have SSL at all.
    security: ConnectionSecurity,
}

/////////////////////////////////////////////////
// Handle Default Values Carefully
/////////////////////////////////////////////////
// DON'T: Implement `Default` without consideration
// #[derive(Default)]  // Might create invalid states!
// struct ServerConfig {
//     port: u16,      // Will be 0, which isn't a valid port!
//     max_connections: usize,
//     timeout_seconds: u64,
// }

// DO: Make Default meaningful or don't implement it
#[derive(Debug, Clone, Copy)]
struct Port(u16);

impl Port {
    pub fn new(value: u16) -> Result<Self, Error> {
        if value == 0 {
            return Err(Error);
        }
        Ok(Port(value))
    }
}

struct ServerConfig {
    port: Port,
    max_connections: std::num::NonZeroUsize,
    timeout_seconds: std::time::Duration,
}

impl ServerConfig {
    pub fn new(port: Port) -> Self {
        Self {
            port,
            max_connections: std::num::NonZeroUsize::new(100).unwrap(),
            timeout_seconds: std::time::Duration::from_secs(30),
        }
    }
}