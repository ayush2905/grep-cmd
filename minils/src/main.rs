use std::env;
use std::process;

use minils::Config;

fn main() {


    let args: Vec<String> = env::args().collect();

    // let dirname = &args[1];
    
    // let current_dir = env::current_dir().unwrap();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing args:{:?}", err);
        process::exit(1);
    });

    if let Err(e) = minils::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }


    // let entries = fs::read_dir(dirname).unwrap();

    // for entry in entries {
    //     println!("{:?}", entry.unwrap().path());
    // }
}
