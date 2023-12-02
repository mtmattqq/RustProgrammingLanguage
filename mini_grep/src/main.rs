use std::env;
use std::process;

use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments : {}", err);
        process::exit(1);
    });
    
    eprintln!("Args : {:?}", args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application fail : {}", e);
        process::exit(1);
    }
}



