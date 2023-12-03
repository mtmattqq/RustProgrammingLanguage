use std::env;
use std::process;

use mini_grep::Config;

fn main() {    
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments : {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application fail : {}", e);
        process::exit(1);
    }
}



