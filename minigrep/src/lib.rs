use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        case_insensitive_search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())

}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = &query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "
Rust
safe, fast, productive
Pick three.";
        println!("{:?}", search(query, contents));
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }

    #[test] 
    fn two_result() {
        let query = "RuSt";
        let contents = "
Rust
safe, fast, productive
Pick three.";
        // println!("{:?}", insensitive_search(query, contents));
        assert_eq!(vec!["Rust"], case_insensitive_search(query, contents));
    }

}