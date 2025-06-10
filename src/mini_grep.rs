use std::{env, fs, process};

pub fn mini_grep(args: &Vec<String>) {
    dbg!(args);
    
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    // Because 'run' returns () in the success case, we only care about detecting an error,
    // so we don't need unwrap_or_else to return the unwrapped value, which would only be ().
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// The function will return a type that implements the Error trait,
// bug we don't have to specify what particular type the return value will be.
// Using () like this is the idiomatic way to indicate that we're calling function for its
// side effects only; it doesn't return a value we need.
fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.file_path)?; // '?' lets the caller handle errors
    
    let results = if config.ignore_case { 
        search_case_insensitive(&config.query, &contents)
    } else { 
        search_case_sensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return  Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

// The data returned by the function will live as long as the data
// passed into this function in the 'contents' argument.
fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // '\' after the opening double quote tells Rust not to put a newline character
        // at the beginning of the contents of this string literal.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
