use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query =  match args.next() {
            None => return Err("Did not get a query string"),
            Some(arg) => arg,
        };

        let filename = match args.next() {
            None => return Err("Did not get a filename string"),
            Some(arg) => arg,
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search_case_sensitive(query: &str, contents: & str) -> Vec<String> {
    contents.lines()
        .filter(|line| line.contains(query))
        .map(String::from)
        .collect()
}

pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .map(String::from)
        .collect()
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    if config.case_sensitive {
        return Ok(search_case_sensitive(&config.query, &contents))
    }
    Ok(search_case_insensitive(&config.query, &contents))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_insensitive() {
        let query = "hell";
        let contents = "\
Test string;
This is Hell;
hello world;
        ";
        assert_eq!(
            vec!["This is Hell;", "hello world;"],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn result_sensitive() {
        let query = "hell";
        let contents = "\
Test string;
This is Hell;
hello world;
        ";
        assert_eq!(
            vec!["hello world;"],
            search_case_sensitive(query, contents)
        );
    }
}
