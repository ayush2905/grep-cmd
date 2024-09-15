use std::env;
use std::process;

use minils::Config;
use minils::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing args:{:?}", err);
        process::exit(1);
    });

    if let Err(e) = minils::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
