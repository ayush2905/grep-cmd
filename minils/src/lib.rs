use std::error::Error;
use std::fs;

pub struct Config {
    pub directory: String,
    pub current: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str> {
        // let mut current = "false";
        if args.len() < 2 {
            let current = "true".to_string();
            let directory = ".".to_string();
            return Ok(Config{directory, current});
        } else {
            let current = "false".to_string();
            let directory = args[1].clone();
            return Ok(Config{directory, current});
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let metadata = fs::metadata(config.directory).unwrap();

    if metadata.is_file() {
        println!("It is a file");
    } else {
        println!("Its a directory");
    }

    Ok(())
}

// pub fn list_files(config: Config) -> Vec<String> {

// }