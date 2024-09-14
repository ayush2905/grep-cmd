use std::fs;
use std::path;
use std::env;

fn main() {


    let args: Vec<String> = env::args().collect();

    let dirname = &args[1];

    let current_dir = env::current_dir().unwrap();

    println!("{:?}", args);


    let entries = fs::read_dir(dirname).unwrap();

    for entry in entries {
        println!("{:?}", entry.unwrap().path());
    }
}
