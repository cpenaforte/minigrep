use std::{fs, io};

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
        ignore_case: bool,
    ) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query_str) => query_str,
            None => return Err("Query not given"),
        };

        let file_path = match args.next() {
            Some(file_path_str) => file_path_str,
            None => return Err("File path not given"),
        };

        let ignore_case = ignore_case;

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub fn run(config: Config) -> Result<(), io::Error> {
    let file_content = fs::read_to_string(&(config.file_path))?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &file_content) {
            println!("{line}");
        }

        return Ok(());
    }

    for line in search(&config.query, &file_content) {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
