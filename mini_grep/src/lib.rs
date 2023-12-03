/// Search a word in a content
/// 
/// ## Function
/// ```
/// let config = Config::new(env::args()).unwrap_or_else(|err| {
///     eprintln!("Problem parsing arguments : {}", err);
///     process::exit(1);
/// });
/// ```
/// 

use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insentive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String, 
    pub filename: String, 
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duck here three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insentive(query, contents)
        )
    }
}
