<<<<<<< HEAD
use std::fs;
use std::error::Error;
=======
use std::error::Error;
use std::fs;

>>>>>>> 03d35c9e44796af796567d495ad2a041c2b781e5
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

<<<<<<< HEAD

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    // println!("With text:\n{}", contents);
=======
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
>>>>>>> 03d35c9e44796af796567d495ad2a041c2b781e5

    Ok(())
}

<<<<<<< HEAD
=======
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

>>>>>>> 03d35c9e44796af796567d495ad2a041c2b781e5
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
<<<<<<< HEAD
    fn one_result() {
        let query = "dict";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
=======
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.";
    }
}
>>>>>>> 03d35c9e44796af796567d495ad2a041c2b781e5
